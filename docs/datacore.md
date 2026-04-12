# Datacore

## Overview

The **Datacore** is a massive binary data table containing almost all gameplay, entity, item, tag, component, and metadata for Star Citizen. It is essentially a high-performance object database storing records and all their complex relationships.

The format is versioned and highly structured, with strong typing for all fields and objects.

## High-Level Structure

A **datacore** file is divided into these primary sections \(in order\):

1. **Header:** Metadata, version, and section counts.
2. **Structure Definitions:** Describes types/classes.
3. **Property Definitions:** Describes fields for structures.
4. **Enum Definitions:** Enumerations for types/fields.
5. **Data Mapping Definitions:** Links structures to their instances.
6. **Record Definitions:** All high-level objects/records in the datacore.
7. **Value Arrays:** Flat arrays for each primitive and reference data type.
8. **String Tables:** Two tables for string deduplication \(names, filenames, etc.\).
9. **Structure Instances:** Raw binary data for each instance of every structure.
10. **\(Possible additional tables depending on version.\)**

All sections are tightly packed; offsets are calculated from the header and counts.

***
## Datacore Header Table

The **header** is always the first structure in a datacore file. It is a fixed-length table of 132 bytes \(33 fields × 4 bytes each, except for 4 x uint16\).

**All values are little-endian.**

| **Offset** | **Field name** | Type | **Purpose / Meaning** |
| --- | --- | --- | --- |
| 0x00 | unknown0 | uint32 | Unknown, often zero. May be a signature or reserved. |
| 0x04 | version | uint32 | Datacore format version. Determines structures used later \(V5/V6\). |
| 0x08 | unknown1 | uint16 | Unknown |
| 0x0A | unknown2 | uint16 | Unknown |
| 0x0C | unknown3 | uint16 | Unknown |
| 0x0E | unknown4 | uint16 | Unknown |
| 0x10 | structure\_definition\_count | uint32 | Number of structure definitions \(types/classes\). |
| 0x14 | property\_definition\_count | uint32 | Number of property definitions \(fields per structure\). |
| 0x18 | enum\_definition\_count | uint32 | Number of enum definitions. |
| 0x1C | data\_mapping\_definition\_count | uint32 | Number of data mapping definitions \(structure-instance mapping\). |
| 0x20 | record\_definition\_count | uint32 | Number of records \(top-level objects: entities, items, tags, etc.\). |
| 0x24 | boolean\_count | uint32 | Total count of bools in the value array section. |
| 0x28 | int8\_count | uint32 | Total count of int8 values. |
| 0x2C | int16\_count | uint32 | Total count of int16 values. |
| 0x30 | int32\_count | uint32 | Total count of int32 values. |
| 0x34 | int64\_count | uint32 | Total count of int64 values. |
| 0x38 | uint8\_count | uint32 | Total count of uint8 values. |
| 0x3C | uint16\_count | uint32 | Total count of uint16 values. |
| 0x40 | uint32\_count | uint32 | Total count of uint32 values. |
| 0x44 | uint64\_count | uint32 | Total count of uint64 values. |
| 0x48 | float\_count | uint32 | Total count of float values. |
| 0x4C | double\_count | uint32 | Total count of double values. |
| 0x50 | guid\_count | uint32 | Total count of GUIDs. |
| 0x54 | string\_count | uint32 | Total count of string references. |
| 0x58 | locale\_count | uint32 | Total count of locale references. |
| 0x5C | enum\_count | uint32 | Total count of enum choices. |
| 0x60 | strong\_value\_count | uint32 | Total count of "strong pointer" references. |
| 0x64 | weak\_value\_count | uint32 | Total count of "weak pointer" references. |
| 0x68 | reference\_count | uint32 | Total count of "reference" entries. |
| 0x6C | enum\_option\_name\_count | uint32 | Total count of enum value names. |
| 0x70 | text\_length | uint32 | Length in bytes of the primary string table \(following values\). |
| 0x74 | text\_length2 | uint32 | Length in bytes of the secondary string table \(v6\+, can be zero\). |

### Field Details and Decoding Notes

- **`version`:**
    - Crucial for determining which layouts and lookup strategies to use.
    - Version 5: one string table; version 6\+: two string tables.
- **All `\*\_count` fields:**
    - Tell you how many entries to read for each subsequent array or table in the file.
- **`text\_length`, `text\_length2`:**
    - Offsets and lengths for string tables; needed for decoding strings for names, filenames, and other text fields.
    - For older versions, `text\_length2` may be zero.
- **Unknowns:**
    - Some `unknown\*` fields are placeholders or may have legacy use. Typically safe to ignore.

***
## Structure Definitions

### Purpose

The **Structure Definitions** table describes all the _types_ or _classes_ used in the datacore.

Each structure tells you:

- Its name \(as a string offset\)
- Its parent \(for inheritance\)
- The number of properties \(fields\) it has
- Where its properties start in the Property Definitions table
- Its internal type/class code

This section is crucial: it defines the "schema" for all the binary object data found later.

### Layout

Each structure definition is a fixed-size record, immediately following the header.

| **Offset** | Field | Type | **Description** |
| --- | --- | --- | --- |
| \+0 | name\_offset | uint32 | Offset into string table for structure name |
| \+4 | parent\_index | uint32 | Parent structure index \(0xFFFFFFFF if none\) |
| \+8 | property\_count | uint16 | Number of properties for this structure |
| \+10 | first\_property\_index | uint16 | Index into property\_definitions array |
| \+12 | node\_type | uint32 | Internal type/class code \(rarely used externally\) |

Each entry is 16 bytes long.

### Decoding Steps

1. **Read all structure definitions:**
    - There are `structure\_definition\_count` of these \(from the header\).
    - Read each as 16 bytes, parse the fields.
2. **For each structure:**
    - `name\_offset`: use to fetch the name from the string table.
    - `parent\_index`: if not 0xFFFFFFFF, this structure inherits properties from the parent \(recursively\).
    - `property\_count` and `first\_property\_index`:
        - Properties for this structure are found in the property definitions array, from `first\_property\_index` to `first\_property\_index \+ property\_count - 1`.
    - `node\_type`: usually an internal code; can be ignored for most reverse engineering purposes.

### Inheritance

- If a structure has a parent, its full schema is the parent's properties **plus** its own properties.
- This allows deep, complex inheritance hierarchies.

***
## Property Definitions

### Purpose

The **Property Definitions** table describes every _field_ of every structure/class defined in the datacore.

It specifies:

- The field's name \(string offset\)
- The type of the field \(from the `DataTypes` enum\)
- The way the field is stored \(conversion type; e.g., direct value, array, reference\)
- Which structure this property belongs to

**Properties are grouped by structure** \(each structure points to its first property and a count\).

---
### Layout

Each property definition is a fixed-size record, directly following the structure definitions.

| Offset | Field | Type | **Description** |
| --- | --- | --- | --- |
| \+0 | name\_offset | uint32 | Offset into string table for property name |
| \+4 | structure\_index | uint16 | Index into structure\_definitions array |
| \+6 | data\_type | uint16 | Type of property value \(DataTypes enum\) |
| \+8 | conversion\_type | uint16 | How property is stored \(ConversionTypes enum\) |
| \+10 | padding | uint16 | Unused \(always zero/padding for alignment\) |

Each entry is 12 bytes long.

### Enums Used

[Enums](https://whimsical.com/JCJrw2kXYx6i2YLauxpyYC)

- [**DataTypes**](https://whimsical.com/enums-JCJrw2kXYx6i2YLauxpyYC@2bsEvpTYFZsxPbd5wXWmy3AqN8BXaUvHTq1)
- [**ConversionTypes**](https://whimsical.com/enums-JCJrw2kXYx6i2YLauxpyYC@2bsEvpTYFZsxPM8jia3RN9dt7DFrQViMyfi)
    - `Attribute` = 0: direct field
    - `ComplexArray`, `SimpleArray`, `ClassArray`: various array types

### Decoding Steps

1. **Read all property definitions:**
    - There are `property\_definition\_count` of these \(from the header\).
    - Read each as 12 bytes, parse the fields.
2. **For each property:**
    - `name\_offset`: use to fetch the property name from the string table.
    - `structure\_index`: links to the parent structure.
    - `data\_type`: tells you which value array or structure to use when reading this property.
    - `conversion\_type`: tells you _how_ to interpret the value.

### How Properties Are Used

- When reading a structure instance, **walk through its properties \(in order\)**:
    - For each property:
        - Use its type and conversion type to read the appropriate number of bytes from the instance data
        - For arrays: conversion type tells you to read a count/index pair, then a slice from the correct value array
        - For references: conversion type and data type tell you which value array to look in \(e.g., GUIDs, references, strong pointers\)

### Practical Example

Suppose you’re decoding an “EntityClassDefinition” structure instance.

- You get the structure’s property list from its structure definition.
- For each property:
    - Use its `data\_type` and `conversion\_type` to know how to parse from the instance data.
    - For a string field, look up the offset in the string table.
    - For a pointer, look up the referenced instance \(by index or GUID\).
    - For an array, use count/index \(see decoding code in your classes\).

---
### Special Note: Recursion

Because structures can contain nested structures, pointers, or arrays of other structures, you may need to recursively decode structure instances using the property definitions.

***
## Enum Definitions

### Purpose

**Enum Definitions** describe all enumeration types used in the datacore.

Each enum definition specifies:

- Its name \(as a string offset\)
- How many values it has
- Where in the enum value table its options begin

This allows properties to have enumerated values instead of arbitrary data.

---
### Layout

Each enum definition is a fixed-size record, directly following the property definitions.

| Offset | Field | Type | **Description** |
| --- | --- | --- | --- |
| \+0 | name\_offset | uint32 | Offset in string table for enum name |
| \+4 | value\_count | uint16 | Number of values in this enum |
| \+6 | first\_value\_index | uint16 | First index into enum value names table |

Each entry is 8 bytes long.

### Enum Value Table

- **Enum values themselves** are generally references into the _enum value name table_ \(a section of the string table or its own table, depending on version\).

### Decoding Steps

1. Read all enum definitions \(there are `enum\_definition\_count` of them, from the header\).
2. For each enum:
    - Name is fetched from string table via `name\_offset`.
    - Values are found by slicing from the enum value names table:
        - `enum\_value\_names\[first\_value\_index : first\_value\_index \+ value\_count\]`.

***
## Data Mapping Definitions

### Purpose

**Data Mapping Definitions** tell you:

- For each structure/class, how many binary instances exist and where to find them
- This is how you know how many of each “object” exist in the datacore and where their binary data starts

There are two formats depending on version: 16-bit \(pre-v5\) and 32-bit \(v5\+\).

---
### Layout

Table Representation \(v6\+ uses 32-bit\)

| Offset | **Field** | Type | **Description** |
| --- | --- | --- | --- |
| \+0 | structure\_count | u16/u32 | Number of instances of this structure |
| \+2/\+4 | structure\_index | u16/u32 | Index  |

Entry size: 4 bytes \(v16\) or 8 bytes \(v32\).

### Decoding Steps

1. There are `data\_mapping\_definition\_count` of these.
2. For each mapping:
    - `structure\_index`: which structure this is for
    - `structure\_count`: how many instances exist in the file
3. The structure instance section \(after string tables\) contains `structure\_count` instances, each of size `structure\_def.calculated\_data\_size`.

***
## Record Definitions

### Purpose

The **Record Definitions** section contains all the high-level objects in the datacore:

- Entities, ships, items, manufacturers, tags, components, etc.

A _Record_ is a reference to a specific data instance, and provides:

- The name and filename of the record \(both string offsets\)
- The type of the record \(via structure index\)
- The binary data for the record \(via structure and instance index\)
- Its unique ID \(GUID\)

Records are the main entry points for working with the game’s object database.

---
### Layout

Each record definition is a fixed-size structure, following the data mapping definitions.

| Offset | Field | Type | **Description** |
| --- | --- | --- | --- |
| \+0 | name\_offset | uint32 | Offset in \(usually secondary\) string table for record name |
| \+4 | filename\_offset | uint32 | Offset in string table for filename/source path |
| \+8 | structure\_index | uint32 | Index into structure\_definitions \(what "type" this record is\) |
| \+12 | id | GUID\(16B\) | Globally unique identifier for the record |
| \+28 | instance\_index | uint16 | Which structure instance this record represents |
| \+30 | other\_index | uint16 | Unknown \(possibly subclass or linking index\) |

**Each entry is 32 bytes long.**

There are `record\_definition\_count` entries.

---
### How Record Definitions Are Used

- **name\_offset**:
    - Used with the secondary string table in v6\+, or primary in earlier versions.
    - Often namespaced, e.g., `"EntityClassDefinition.Aurora\_ES"`.
    - Code usually strips the type prefix for a “clean” name.
- **filename\_offset**:
    - Path to the record’s source definition \(often `.xml` or similar\), used for lookups.
- **structure\_index**:
    - Which structure definition to use for decoding this record’s instance data.
- **id**:
    - A unique GUID for cross-referencing records.
- **instance\_index**:
    - Within the mapping for this structure, tells you which instance this record uses.
- **other\_index**:
    - Usually ignored, rarely used outside the engine.

---
### Decoding Steps

1. Read all record definitions \(there are `record\_definition\_count` of them\).
2. For each record:
    - `name\_offset`: fetch name from the correct string table.
    - `filename\_offset`: fetch filename from the string table.
    - `structure\_index`: get the structure type.
    - `id`: store for cross-referencing and lookup.
    - `instance\_index`: this is the Nth instance of the structure for this record.
3. Use the **data mapping definitions** \(from previous section\) to know where the structure’s instance data lives.

---
### Practical Notes

- **Cross-reference**: All references and pointers in the datacore eventually lead back to these records \(often by GUID\).
- **Hierarchy:** The "type" and "name" allow you to classify records \(e.g., “EntityClassDefinition” for a ship\).
- **Lookup:** Efficient lookup by GUID, name, or filename is common for tools and editors.

***
## Value Arrays

### Purpose

The **Value Arrays** section stores all primitive and reference values used by the properties of structure instances.

These arrays are:

- Flat \(not interleaved\)
- Typed \(one array per type: booleans, int8, uint32, float, GUIDs, string references, etc.\)
- Indexed: Properties in structures reference entries here **by index** \(not by direct value\).

The layout and counts for each array are **specified in the header**.

---
### Types of Value Arrays

**Each array is present in this order, directly following the record definitions:**

| Array Name | **Count Field \(header\)** | Type | **Description** |
| --- | --- | --- | --- |
| booleans | boolean\_count | bool | Packed booleans |
| int8 | int8\_count | int8 | 8-bit signed integers |
| int16 | int16\_count | int16 | 16-bit signed integers |
| int32 | int32\_count | int32 | 32-bit signed integers |
| int64 | int64\_count | int64 | 64-bit signed integers |
| uint8 | uint8\_count | uint8 | 8-bit unsigned integers |
| uint16 | uint16\_count | uint16 | 16-bit unsigned integers |
| uint32 | uint32\_count | uint32 | 32-bit unsigned integers |
| uint64 | uint64\_count | uint64 | 64-bit unsigned integers |
| float | float\_count | float32 | 32-bit floats |
| double | double\_count | float64 | 64-bit floats |
| GUIDs | guid\_count | GUID\(16B\) | 128-bit unique IDs |
| string refs | string\_count | uint32 | Offset into primary string table |
| locale refs | locale\_count | uint32 | Offset into string table |
| enum choices | enum\_count | uint32 | Offset into enum value table |
| strong ptrs | strong\_value\_count | \(struct\) | Structure and instance indices |
| weak ptrs | weak\_value\_count | \(struct\) | Structure and instance indices |
| references | reference\_count | \(struct\) | Instance index \+ GUID |
| enum names | enum\_option\_name\_count | uint32 | Offset into enum value names table |

Each array is simply the sequence of that type’s values, packed together, and read in the order listed above.

---
### How They’re Used

- When decoding a property of a structure instance:
    - The property’s type \(from `PropertyDefinition`\) tells you which value array to use.
    - The property’s value is often an **index into the relevant value array**.
    - For arrays \(of numbers, strings, GUIDs, etc.\), there is usually a pair `\(count, start\_index\)`, and you take a slice from the value array.

---
### Example \(Pseudocode\)

```
// Read each value array \(counts from header\):
bool\_values    = read\_array\(bool,    header.boolean\_count\)
int8\_values    = read\_array\(int8,    header.int8\_count\)
...
guid\_values    = read\_array\(GUID,    header.guid\_count\)
string\_offsets = read\_array\(uint32,  header.string\_count\)
...

// To resolve a property:
if prop.data\_type == DataTypes.StringRef:
    string\_offset = string\_offsets\[prop\_value\_index\]
    // decode from string table using offset
elif prop.data\_type == DataTypes.Float:
    value = float\_values\[prop\_value\_index\]
elif prop.data\_type == DataTypes.Reference:
    ref = reference\_structs\[prop\_value\_index\]
    // resolve by GUID, etc.
```
***
## String Tables

### Purpose

The **String Tables** deduplicate and store all textual content:

- Structure names, property names, record names, filenames, enum values, etc.

There are two string tables:

1. Primary String Table

- Size: `text\_length` \(from header\)
- Stores most string content, especially for pre-v6 files.
- Properties and some names reference this table using an offset.

1. Secondary String Table

- Size: `text\_length2` \(from header\)
- Used in datacore v6\+ for some record names \(typically those with a type prefix, e.g., `"EntityClassDefinition.Aurora\_ES"`\).
- Not always present \(`text\_length2` may be zero in old files\).

### Layout

- **Both tables:** Packed, null-terminated UTF-8 strings \(`b'FooBar\x00NextString\x00...'`\)
- **Offsets:** All references into the tables are relative to the start of the respective table.

### Usage

- To get a string:
    - Use the offset value \(from value array, property, etc.\)
    - Seek to that offset in the appropriate string table
    - Read bytes until null terminator \(`0x00`\)
    - Decode as UTF-8

### Example \(Pseudocode\)

```
primary\_strings   = file\[primary\_string\_table\_offset : primary\_string\_table\_offset \+ text\_length\]
secondary\_strings = file\[secondary\_string\_table\_offset : secondary\_string\_table\_offset \+ text\_length2\]

def get\_string\(table, offset\):
    end = table.index\(0x00, offset\)
    return table\[offset:end\].decode\('utf-8'\)

// Usage:
name = get\_string\(primary\_strings, name\_offset\)
filename = get\_string\(primary\_strings, filename\_offset\)
if version >= 6:
    record\_name = get\_string\(secondary\_strings, name\_offset2\)
```
### Practical Notes

- Always check the `version` field in the header to know which string table to use for a given field.
- Offsets are **not absolute file offsets**—they’re relative to the start of the string table.

***
## Structure Instances \(Object Data Blobs\)

### Purpose

This section contains the **raw binary data** for every instance of every structure/class defined in the datacore.

- Each instance represents an object’s actual property values.
- The layout of each instance is defined by its `StructureDefinition` and corresponding `PropertyDefinitions`.

### Layout

- The structure instance data **immediately follows** the value arrays and string tables.
- There is no delimiter: the number and size of instances per structure type are given by the Data Mapping Definitions and Structure Definitions.
- For each structure type:
    - The corresponding Data Mapping Definition gives:
        - `structure\_index`: which structure type
        - `structure\_count`: how many instances
    - The size of each instance is computed from the sum of all its properties’ storage requirements \(**including inherited properties**\).

### Calculating Offsets

For each structure type:

- The instance section for this structure is a contiguous block:
    - Size = `structure\_count` × `structure\_instance\_size`
- For a record, its `instance\_index` \(from the Record Definition\) gives which instance in this block to use.

You need to keep a running offset while reading all structure instances in sequence.

### Decoding an Instance

1. **Get the Structure Definition** for the structure type \(by index\).
2. **Build the Full Property List** for the structure \(recursively include parent properties first\).
3. **For each property \(in order\):**
    - Use its `data\_type` and `conversion\_type` to determine how many bytes to read and how to interpret them.
    - If it’s a direct value, read the value and resolve from the corresponding value array \(by index\).
    - If it’s an array, read `\(count, start\_index\)`, then slice the relevant value array.
    - If it’s a class, pointer, or reference, read the reference index/GUID and use it to fetch the target instance or record.
    - If it’s a string, resolve the string from the string table using the offset read.
    - If it’s an embedded structure/class, decode it recursively.

**Example \(Pseudocode\)**

```
// Pseudocode for decoding a record instance
record = record\_definitions\[record\_index\]
struct\_def = structure\_definitions\[record.structure\_index\]
instance\_offset = compute\_instance\_offset\(struct\_def, record.instance\_index\)
property\_list = build\_full\_property\_list\(struct\_def\)

cursor = instance\_offset
object\_data = \{\}
for prop in property\_list:
    switch prop.conversion\_type:
        case Attribute: // direct value
            value\_index = read\_appropriate\_type\(cursor, prop.data\_type\)
            value = value\_arrays\[prop.data\_type\]\[value\_index\]
            object\_data\[prop.name\] = value
            cursor \+= sizeof\(prop.data\_type\)
        case SimpleArray or ComplexArray:
            count = read\_uint16\(cursor\)
            start\_index = read\_uint32\(cursor \+ 2\)
            values = value\_arrays\[prop.data\_type\]\[start\_index : start\_index \+ count\]
            object\_data\[prop.name\] = values
            cursor \+= 6
        case Class:
            class\_instance\_index = read\_uint32\(cursor\)
            class\_struct = structure\_definitions\[prop.class\_structure\_index\]
            class\_data = decode\_instance\(class\_struct, class\_instance\_index\)
            object\_data\[prop.name\] = class\_data
            cursor \+= 4
        case Reference, StrongPointer, WeakPointer:
            // read reference index or GUID, resolve as needed
            // \(details depend on type\)
            ...
        // \(Other conversion types as needed\)
    end switch
```
- The specific number of bytes to read per property depends on the property’s type and conversion type.
- Array properties will have a count and a start index.
- For pointer/reference types, you often read a structure index \+ instance index \(sometimes as a struct\).

### Recursion

- If a property is itself a structure/class or an array of them, **decode recursively** using the appropriate structure definition and instance index.

### Property Value Resolution

- **Primitive values**: Use the index from the instance data to look up the actual value in the corresponding value array.
- **Strings**: Use the index/offset to extract the string from the appropriate string table.
- **References**: Use GUID or index to look up the corresponding record or instance.

### Putting It All Together

1. **To decode a record \(object\):**
    - Find the record definition \(by GUID, name, filename, etc.\).
    - Determine the structure type and instance index.
    - Locate the binary instance data in the structure instance section.
    - Decode all properties according to their types, using value arrays, string tables, and, if needed, recursing for nested structures.

### Practical Example \(Walkthrough\)

Let’s say you want to decode a ship entity:

- Look up the `Record` for the ship.
- Get its structure and instance index.
- Find where in the structure instances section the data for this instance starts.
- For each property \(and inherited properties\), read the value\(s\) as directed.
- If a property is a manufacturer reference, use the value as an index into the reference array, then use the GUID there to look up the manufacturer’s `Record`.
- If a property is an array of tags, read the count and start index, then fetch those tags from the value array, then look up each tag’s string in the string table.

### Final Notes

- **Alignment:** All fields are packed, with no padding unless forced by alignment \(depends on type and platform, but usually 4-byte-aligned\).
- **You need the full schema:** You must use the property and structure definitions, not just the binary blobs, to interpret structure instance data correctly.
- **Recursion and pointers:** Many properties are references to other objects or arrays of references—so robust tools must support pointer chasing and recursive decoding.

# Enums

# Data Types

| **Name** | **Value** |
| --- | --- |
| Reference | 0x0310 |
| WeakPointer | 0x0210 |
| StrongPointer | 0x0110 |
| Class | 0x0010 |
| EnumChoice | 0x000F |
| GUID | 0x000E |
| Locale | 0x000D |
| Double | 0x000C |
| Float | 0x000B |
| StringRef | 0x000A |
| UInt64 | 0x0009 |
| UInt32 | 0x0008 |
| UInt16 | 0x0007 |
| UInt8 | 0x0006 |
| Int64 | 0x0005 |
| Int32 | 0x0004 |
| Int16 | 0x0003 |
| Int8 | 0x0002 |
| Boolean | 0x0001 |

# Conversion Types

| **Name** | **Value** |
| --- | --- |
| Attribute | 0 |
| ComplexArray | 1 |
| SimpleArray | 2 |
| ClassArray | 3 |

# String Size

| **Name** | **Value** |
| --- | --- |
| Int8 | 1 |
| Int16 | 2 |
| Int32 | 4 |
