pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type size_t = usize;
pub type ssize_t = usize;
pub type char = u8;
pub type int = i16;
pub type void = ();

pub struct InputBuffer {
    pub buffer: *mut char,
    pub buffer_length: size_t,
    pub input_length: ssize_t,
}

pub enum ExecuteResult {
    EXECUTE_SUCCESS,
    EXECUTE_DUPLICATE_KEY,
}
  
pub enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND
}
  
pub enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_NEGATIVE_ID,
    PREPARE_STRING_TOO_LONG,
    PREPARE_SYNTAX_ERROR,
    PREPARE_UNRECOGNIZED_STATEMENT
}

pub enum StatementType { 
    STATEMENT_INSERT, 
    STATEMENT_SELECT 
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 225;

pub struct Row {
    pub id: uint32_t,
    pub username: [char; COLUMN_USERNAME_SIZE + 1],
    pub email: [char; COLUMN_EMAIL_SIZE + 1],
}

pub struct Statement {
    pub type_0: StatementType,
    pub row_to_insert: Row,
}

pub static mut ID_SIZE: uint32_t = 0;
pub static mut USERNAME_SIZE: uint32_t = 0;
pub static mut EMAIL_SIZE: uint32_t = 0;
pub static mut ID_OFFSET: uint32_t = 0;
pub static mut USERNAME_OFFSET: uint32_t = 0;
pub static mut EMAIL_OFFSET: uint32_t = 0;
pub static mut ROW_SIZE: uint32_t = 0;
pub static mut PAGE_SIZE: uint32_t = 4096;

const TABLE_MAX_PAGES: usize = 225;

pub struct Pager {
    pub file_descriptor: int,
    pub file_length: uint32_t,
    pub num_pages: uint32_t,
    pub pages: [*mut void; 100],
}
#[derive(Copy, Clone)]
pub struct Table {
    pub pager: *mut Pager,
    pub root_page_num: uint32_t,
}

pub struct Cursor {
    pub table: *mut Table,
    pub page_num: uint32_t,
    pub cell_num: uint32_t,
    pub end_of_table: bool,
}

pub fn print_row(row: *mut Row) {
    unsafe {
    println!("({}, {}, {})",
           (*row).id, (*row).username.as_mut_ptr().as_mut().unwrap(),
           (*row).email.as_mut_ptr().as_mut().unwrap());
    }
}

pub type NodeType = uint32_t;
pub const NODE_LEAF: NodeType = 1;
pub const NODE_INTERNAL: NodeType = 0;

/*
 * Common Node Header Layout
 */
pub static mut NODE_TYPE_SIZE: uint32_t =
    ::std::mem::size_of::<uint8_t>() as uint32_t;
pub static mut NODE_TYPE_OFFSET: uint32_t = 0 as libc::c_int as uint32_t;
pub static mut IS_ROOT_SIZE: uint32_t =
    ::std::mem::size_of::<uint8_t>() as uint32_t;
pub static mut IS_ROOT_OFFSET: uint32_t = unsafe { NODE_TYPE_SIZE };
pub static mut PARENT_POINTER_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as uint32_t;
pub static mut PARENT_POINTER_OFFSET: uint32_t = 0;
pub static mut COMMON_NODE_HEADER_SIZE: uint8_t = 0;

/*
 * Internal Node Header Layout
 */

pub static mut INTERNAL_NODE_NUM_KEYS_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut INTERNAL_NODE_NUM_KEYS_OFFSET: uint32_t =
    unsafe { COMMON_NODE_HEADER_SIZE as uint32_t };
pub static mut INTERNAL_NODE_RIGHT_CHILD_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut INTERNAL_NODE_RIGHT_CHILD_OFFSET: uint32_t = 0;
pub static mut INTERNAL_NODE_HEADER_SIZE: uint32_t = 0;
/*
 * Internal Node Body Layout
 */

pub static mut INTERNAL_NODE_KEY_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut INTERNAL_NODE_CHILD_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut INTERNAL_NODE_CELL_SIZE: uint32_t = 0;
/* Keep this small for testing */
pub static mut INTERNAL_NODE_MAX_CELLS: uint32_t =
    3 as libc::c_int as uint32_t;
/*
 * Leaf Node Header Layout
 */
pub static mut LEAF_NODE_NUM_CELLS_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut LEAF_NODE_NUM_CELLS_OFFSET: uint32_t =
    unsafe { COMMON_NODE_HEADER_SIZE as uint32_t };
pub static mut LEAF_NODE_NEXT_LEAF_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut LEAF_NODE_NEXT_LEAF_OFFSET: uint32_t = 0;
pub static mut LEAF_NODE_HEADER_SIZE: uint32_t = 0;
/*
 * Leaf Node Body Layout
 */
pub static mut LEAF_NODE_KEY_SIZE: uint32_t =
    ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
pub static mut LEAF_NODE_KEY_OFFSET: uint32_t = 0 as libc::c_int as uint32_t;
pub static mut LEAF_NODE_VALUE_SIZE: uint32_t = unsafe { ROW_SIZE };
pub static mut LEAF_NODE_VALUE_OFFSET: uint32_t = 0;
pub static mut LEAF_NODE_CELL_SIZE: uint32_t = 0;
pub static mut LEAF_NODE_SPACE_FOR_CELLS: uint32_t = 0;
pub static mut LEAF_NODE_MAX_CELLS: uint32_t = 0;
pub static mut LEAF_NODE_RIGHT_SPLIT_COUNT: uint32_t = 0;
pub static mut LEAF_NODE_LEFT_SPLIT_COUNT: uint32_t = 0;


pub unsafe fn get_node_type(mut node: *mut libc::c_void) -> NodeType {
    let mut value: uint8_t = *(node.offset(NODE_TYPE_OFFSET as isize) as *mut uint8_t);
    return value as NodeType;
}

pub unsafe fn is_node_root(mut node: *mut libc::c_void) -> bool {
    let mut value: uint8_t =
        *(node.offset(IS_ROOT_OFFSET as isize) as *mut uint8_t);
    return value != 0;
}

pub unsafe fn set_node_root(mut node: *mut libc::c_void,
                                       mut is_root: bool) {
    let mut value: uint8_t = is_root as uint8_t;
    *(node.offset(IS_ROOT_OFFSET as isize) as *mut uint8_t) = value;
}

pub unsafe fn node_parent(mut node: *mut libc::c_void)
 -> *mut uint32_t {
    return node.offset(PARENT_POINTER_OFFSET as isize) as *mut uint32_t;
}

pub unsafe fn internal_node_num_keys(mut node: *mut libc::c_void)
 -> *mut uint32_t {
    return node.offset(INTERNAL_NODE_NUM_KEYS_OFFSET as isize) as
               *mut uint32_t;
}

pub unsafe fn internal_node_right_child(mut node:
                                                       *mut libc::c_void)
 -> *mut uint32_t {
    return node.offset(INTERNAL_NODE_RIGHT_CHILD_OFFSET as isize) as
               *mut uint32_t;
}

pub unsafe fn internal_node_cell(mut node: *mut libc::c_void,
                                            mut cell_num: uint32_t)
 -> *mut uint32_t {
    return node.offset(INTERNAL_NODE_HEADER_SIZE as
                           isize).offset(cell_num.wrapping_mul(INTERNAL_NODE_CELL_SIZE)
                                             as isize) as *mut uint32_t;
}

pub unsafe fn internal_node_child(mut node: *mut libc::c_void,
                                             mut child_num: uint32_t)
 -> *mut uint32_t {
    let mut num_keys: uint32_t = *internal_node_num_keys(node);
    if child_num > num_keys {
        println!("Tried to access child_num {} > num_keys {}", child_num, num_keys);
        std::process::exit(0x0100);
    } else if child_num == num_keys {
        return internal_node_right_child(node)
    } else { return internal_node_cell(node, child_num) };
}

pub unsafe fn internal_node_key(mut node: *mut libc::c_void,
                                           mut key_num: uint32_t)
 -> *mut uint32_t {
    return (internal_node_cell(node, key_num) as
                *mut libc::c_void).offset(INTERNAL_NODE_CHILD_SIZE as isize)
               as *mut uint32_t;
}

pub unsafe fn leaf_node_num_cells(mut node: *mut libc::c_void)
 -> *mut uint32_t {
    return node.offset(LEAF_NODE_NUM_CELLS_OFFSET as isize) as *mut uint32_t;
}

pub unsafe fn leaf_node_next_leaf(mut node: *mut libc::c_void)
 -> *mut uint32_t {
    return node.offset(LEAF_NODE_NEXT_LEAF_OFFSET as isize) as *mut uint32_t;
}

pub unsafe fn leaf_node_cell(mut node: *mut libc::c_void,
                                        mut cell_num: uint32_t)
 -> *mut libc::c_void {
    return node.offset(LEAF_NODE_HEADER_SIZE as
                           isize).offset(cell_num.wrapping_mul(LEAF_NODE_CELL_SIZE)
                                             as isize);
}

pub unsafe fn leaf_node_key(mut node: *mut libc::c_void,
                                       mut cell_num: uint32_t)
 -> *mut uint32_t {
    return leaf_node_cell(node, cell_num) as *mut uint32_t;
}

pub unsafe fn leaf_node_value(mut node: *mut libc::c_void,
                                         mut cell_num: uint32_t)
 -> *mut libc::c_void {
    return leaf_node_cell(node, cell_num).offset(LEAF_NODE_KEY_SIZE as isize);
}

pub unsafe fn get_node_max_key(mut node: *mut core::ffi::c_void)
 -> uint32_t {
    match get_node_type(node) as libc::c_uint {
        0 => {
            return *internal_node_key(node,
                                      (*internal_node_num_keys(node)).wrapping_sub(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint))
        }
        1 => {
            return *leaf_node_key(node,
                                  (*leaf_node_num_cells(node)).wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint))
        }
        _ => { }
    }
    panic!("Reached end of non-void function without returning");
}

pub unsafe fn print_constants() {
    println!("ROW_SIZE: {}", ROW_SIZE);
    println!("COMMON_NODE_HEADER_SIZE: {}", COMMON_NODE_HEADER_SIZE as libc::c_int);
    println!("LEAF_NODE_HEADER_SIZE: {}", LEAF_NODE_HEADER_SIZE);
    println!("LEAF_NODE_CELL_SIZE: {}", LEAF_NODE_CELL_SIZE);
    println!("LEAF_NODE_SPACE_FOR_CELLS: {}", LEAF_NODE_SPACE_FOR_CELLS);
    println!("LEAF_NODE_MAX_CELLS: {}", LEAF_NODE_MAX_CELLS);
}

pub unsafe fn get_page(mut pager: *mut Pager,
                                  mut page_num: uint32_t)
 -> *mut libc::c_void {
    if page_num > 100 as libc::c_int as libc::c_uint {
        println!("Tried to fetch page number out of bounds. {} > {}", page_num,
               100 as libc::c_int);
               std::process::exit(0x0100);
    }
    if (*pager).pages[page_num as usize].is_null() {
        // Cache miss. Allocate memory and load from file.
        let mut page: *mut libc::c_void = malloc(PAGE_SIZE as libc::c_ulong);
        let mut num_pages: uint32_t =
            (*pager).file_length.wrapping_div(PAGE_SIZE);
        // We might save a partial page at the end of the file
        if (*pager).file_length.wrapping_rem(PAGE_SIZE) != 0 {
            num_pages =
                (num_pages as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t
        }
        if page_num <= num_pages {
            lseek((*pager).file_descriptor,
                  page_num.wrapping_mul(PAGE_SIZE) as __off_t,
                  0 as libc::c_int);
            let mut bytes_read: ssize_t =
                read((*pager).file_descriptor, page, PAGE_SIZE as size_t);
            if bytes_read == -(1 as libc::c_int) as libc::c_long {
                println!(b"Error reading file: %d\n\x00" as *const u8 as
                           *const libc::c_char, *__errno_location());
                exit(1 as libc::c_int);
            }
        }
        (*pager).pages[page_num as usize] = page;
        if page_num >= (*pager).num_pages {
            (*pager).num_pages =
                page_num.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
    }
    return (*pager).pages[page_num as usize];
}

pub unsafe fn indent(mut level: uint32_t) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < level {
        println!(b"  \x00" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1)
    };
}

pub unsafe fn print_tree(mut pager: *mut Pager,
                                    mut page_num: uint32_t,
                                    mut indentation_level: uint32_t) {
    let mut node: *mut libc::c_void = get_page(pager, page_num);
    let mut num_keys: uint32_t = 0;
    let mut child: uint32_t = 0;
    match get_node_type(node) as libc::c_uint {
        1 => {
            num_keys = *leaf_node_num_cells(node);
            indent(indentation_level);
            println!(b"- leaf (size %d)\n\x00" as *const u8 as
                       *const libc::c_char, num_keys);
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < num_keys {
                indent(indentation_level.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint));
                println!(b"- %d\n\x00" as *const u8 as *const libc::c_char,
                       *leaf_node_key(node, i));
                i = i.wrapping_add(1)
            }
        }
        0 => {
            num_keys = *internal_node_num_keys(node);
            indent(indentation_level);
            println!(b"- internal (size %d)\n\x00" as *const u8 as
                       *const libc::c_char, num_keys);
            let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
            while i_0 < num_keys {
                child = *internal_node_child(node, i_0);
                print_tree(pager, child,
                           indentation_level.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint));
                indent(indentation_level.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint));
                println!(b"- key %d\n\x00" as *const u8 as *const libc::c_char,
                       *internal_node_key(node, i_0));
                i_0 = i_0.wrapping_add(1)
            }
            child = *internal_node_right_child(node);
            print_tree(pager, child,
                       indentation_level.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint));
        }
        _ => { }
    };
}

pub unsafe fn serialize_row(mut source: *mut Row,
                                       mut destination: *mut libc::c_void) {
    memcpy(destination.offset(ID_OFFSET as isize),
           &mut (*source).id as *mut uint32_t as *const libc::c_void,
           ID_SIZE as libc::c_ulong);
    memcpy(destination.offset(USERNAME_OFFSET as isize),
           &mut (*source).username as *mut [libc::c_char; 33] as
               *const libc::c_void, USERNAME_SIZE as libc::c_ulong);
    memcpy(destination.offset(EMAIL_OFFSET as isize),
           &mut (*source).email as *mut [libc::c_char; 256] as
               *const libc::c_void, EMAIL_SIZE as libc::c_ulong);
}

pub unsafe fn deserialize_row(mut source: *mut libc::c_void,
                                         mut destination: *mut Row) {
    memcpy(&mut (*destination).id as *mut uint32_t as *mut libc::c_void,
           source.offset(ID_OFFSET as isize), ID_SIZE as libc::c_ulong);
    memcpy(&mut (*destination).username as *mut [libc::c_char; 33] as
               *mut libc::c_void, source.offset(USERNAME_OFFSET as isize),
           USERNAME_SIZE as libc::c_ulong);
    memcpy(&mut (*destination).email as *mut [libc::c_char; 256] as
               *mut libc::c_void, source.offset(EMAIL_OFFSET as isize),
           EMAIL_SIZE as libc::c_ulong);
}

pub unsafe fn initialize_leaf_node(mut node: *mut libc::c_void) {
    set_node_type(node, NODE_LEAF);
    set_node_root(node, 0 as libc::c_int != 0);
    *leaf_node_num_cells(node) = 0 as libc::c_int as uint32_t;
    *leaf_node_next_leaf(node) = 0 as libc::c_int as uint32_t;
    // 0 represents no sibling
}

pub unsafe fn initialize_internal_node(mut node:
                                                      *mut libc::c_void) {
    set_node_type(node, NODE_INTERNAL);
    set_node_root(node, 0 as libc::c_int != 0);
    *internal_node_num_keys(node) = 0 as libc::c_int as uint32_t;
}

pub unsafe fn leaf_node_find(mut table: *mut Table,
                                        mut page_num: uint32_t,
                                        mut key: uint32_t) -> *mut Cursor {
    let mut node: *mut libc::c_void = get_page((*table).pager, page_num);
    let mut num_cells: uint32_t = *leaf_node_num_cells(node);
    let mut cursor: *mut Cursor =
        malloc(::std::mem::size_of::<Cursor>() as libc::c_ulong) as
            *mut Cursor;
    (*cursor).table = table;
    (*cursor).page_num = page_num;
    (*cursor).end_of_table = 0 as libc::c_int != 0;
    // Binary search
    let mut min_index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut one_past_max_index: uint32_t = num_cells;
    while one_past_max_index != min_index {
        let mut index: uint32_t =
            min_index.wrapping_add(one_past_max_index).wrapping_div(2 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint);
        let mut key_at_index: uint32_t = *leaf_node_key(node, index);
        if key == key_at_index { (*cursor).cell_num = index; return cursor }
        if key < key_at_index {
            one_past_max_index = index
        } else {
            min_index = index.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
    }
    (*cursor).cell_num = min_index;
    return cursor;
}

pub unsafe fn internal_node_find_child(mut node: *mut libc::c_void,
                                                  mut key: uint32_t)
 -> uint32_t {
    /*
  Return the index of the child which should contain
  the given key.
  */
    let mut num_keys: uint32_t = *internal_node_num_keys(node);
    /* Binary search */
    let mut min_index: uint32_t =
        0 as libc::c_int as uint32_t; /* there is one more child than key */
    let mut max_index: uint32_t = num_keys;
    while min_index != max_index {
        let mut index: uint32_t =
            min_index.wrapping_add(max_index).wrapping_div(2 as libc::c_int as
                                                               libc::c_uint);
        let mut key_to_right: uint32_t = *internal_node_key(node, index);
        if key_to_right >= key {
            max_index = index
        } else {
            min_index = index.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
    }
    return min_index;
}

pub unsafe fn internal_node_find(mut table: *mut Table,
                                            mut page_num: uint32_t,
                                            mut key: uint32_t)
 -> *mut Cursor {
    let mut node: *mut libc::c_void = get_page((*table).pager, page_num);
    let mut child_index: uint32_t = internal_node_find_child(node, key);
    let mut child_num: uint32_t = *internal_node_child(node, child_index);
    let mut child: *mut libc::c_void = get_page((*table).pager, child_num);
    match get_node_type(child) as libc::c_uint {
        1 => { return leaf_node_find(table, child_num, key) }
        0 => { return internal_node_find(table, child_num, key) }
        _ => { }
    }
    panic!("Reached end of non-void function without returning");
}
/*
Return the position of the given key.
If the key is not present, return the position
where it should be inserted
*/

pub unsafe fn table_find(mut table: *mut Table, mut key: uint32_t)
 -> *mut Cursor {
    let mut root_page_num: uint32_t = (*table).root_page_num;
    let mut root_node: *mut libc::c_void =
        get_page((*table).pager, root_page_num);
    if get_node_type(root_node) as libc::c_uint ==
           NODE_LEAF as libc::c_int as libc::c_uint {
        return leaf_node_find(table, root_page_num, key)
    } else { return internal_node_find(table, root_page_num, key) };
}

pub unsafe fn table_start(mut table: *mut Table) -> *mut Cursor {
    let mut cursor: *mut Cursor =
        table_find(table, 0 as libc::c_int as uint32_t);
    let mut node: *mut libc::c_void =
        get_page((*table).pager, (*cursor).page_num);
    let mut num_cells: uint32_t = *leaf_node_num_cells(node);
    (*cursor).end_of_table = num_cells == 0 as libc::c_int as libc::c_uint;
    return cursor;
}

pub unsafe fn cursor_value(mut cursor: *mut Cursor)
 -> *mut libc::c_void {
    let mut page_num: uint32_t = (*cursor).page_num;
    let mut page: *mut libc::c_void =
        get_page((*(*cursor).table).pager, page_num);
    return leaf_node_value(page, (*cursor).cell_num);
}

pub unsafe fn cursor_advance(mut cursor: *mut Cursor) {
    let mut page_num: uint32_t = (*cursor).page_num;
    let mut node: *mut libc::c_void =
        get_page((*(*cursor).table).pager, page_num);
    (*cursor).cell_num =
        ((*cursor).cell_num as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    if (*cursor).cell_num >= *leaf_node_num_cells(node) {
        /* Advance to next leaf node */
        let mut next_page_num: uint32_t = *leaf_node_next_leaf(node);
        if next_page_num == 0 as libc::c_int as libc::c_uint {
            /* This was rightmost leaf */
            (*cursor).end_of_table = 1 as libc::c_int != 0
        } else {
            (*cursor).page_num = next_page_num;
            (*cursor).cell_num = 0 as libc::c_int as uint32_t
        }
    };
}

pub unsafe fn pager_open(mut filename: *const libc::c_char)
 -> *mut Pager {
    let mut fd: libc::c_int =
        open(filename, 0o2 as libc::c_int | 0o100 as libc::c_int,
             0o200 as libc::c_int | 0o400 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        println!(b"Unable to open file\n\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut file_length: off_t =
        lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    let mut pager: *mut Pager =
        malloc(::std::mem::size_of::<Pager>() as libc::c_ulong) as *mut Pager;
    (*pager).file_descriptor = fd;
    (*pager).file_length = file_length as uint32_t;
    (*pager).num_pages =
        (file_length / PAGE_SIZE as libc::c_long) as uint32_t;
    if file_length % PAGE_SIZE as libc::c_long !=
           0 as libc::c_int as libc::c_long {
        println!(b"Db file is not a whole number of pages. Corrupt file.\n\x00"
                   as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < 100 as libc::c_int as libc::c_uint {
        (*pager).pages[i as usize] = 0 as *mut libc::c_void;
        i = i.wrapping_add(1)
    }
    return pager;
}

pub unsafe fn db_open(mut filename: *const libc::c_char)
 -> *mut Table {
    let mut pager: *mut Pager = pager_open(filename);
    let mut table: *mut Table =
        malloc(::std::mem::size_of::<Table>() as libc::c_ulong) as *mut Table;
    (*table).pager = pager;
    (*table).root_page_num = 0 as libc::c_int as uint32_t;
    if (*pager).num_pages == 0 as libc::c_int as libc::c_uint {
        // New database file. Initialize page 0 as leaf node.
        let mut root_node: *mut libc::c_void =
            get_page(pager, 0 as libc::c_int as uint32_t);
        initialize_leaf_node(root_node);
        set_node_root(root_node, 1 as libc::c_int != 0);
    }
    return table;
}

pub unsafe fn new_input_buffer() -> *mut InputBuffer {
    let mut input_buffer: *mut InputBuffer =
        malloc(::std::mem::size_of::<InputBuffer>() as libc::c_ulong) as
            *mut InputBuffer;
    (*input_buffer).buffer = 0 as *mut libc::c_char;
    (*input_buffer).buffer_length = 0 as libc::c_int as size_t;
    (*input_buffer).input_length = 0 as libc::c_int as ssize_t;
    return input_buffer;
}

pub unsafe fn print_prompt() {
    println!(b"db > \x00" as *const u8 as *const libc::c_char);
}

pub unsafe fn read_input(mut input_buffer: *mut InputBuffer) {
    let mut bytes_read: ssize_t =
        getline(&mut (*input_buffer).buffer,
                &mut (*input_buffer).buffer_length, stdin);
    if bytes_read <= 0 as libc::c_int as libc::c_long {
        println!(b"Error reading input\n\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    // Ignore trailing newline
    (*input_buffer).input_length =
        bytes_read - 1 as libc::c_int as libc::c_long;
    *(*input_buffer).buffer.offset((bytes_read -
                                        1 as libc::c_int as libc::c_long) as
                                       isize) =
        0 as libc::c_int as libc::c_char;
}

pub unsafe fn close_input_buffer(mut input_buffer:
                                                *mut InputBuffer) {
    free((*input_buffer).buffer as *mut libc::c_void);
    free(input_buffer as *mut libc::c_void);
}

pub unsafe fn pager_flush(mut pager: *mut Pager,
                                     mut page_num: uint32_t) {
    if (*pager).pages[page_num as usize].is_null() {
        println!(b"Tried to flush null page\n\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut offset: off_t =
        lseek((*pager).file_descriptor,
              page_num.wrapping_mul(PAGE_SIZE) as __off_t, 0 as libc::c_int);
    if offset == -(1 as libc::c_int) as libc::c_long {
        println!(b"Error seeking: %d\n\x00" as *const u8 as *const libc::c_char,
               *__errno_location());
        exit(1 as libc::c_int);
    }
    let mut bytes_written: ssize_t =
        write((*pager).file_descriptor, (*pager).pages[page_num as usize],
              PAGE_SIZE as size_t);
    if bytes_written == -(1 as libc::c_int) as libc::c_long {
        println!(b"Error writing: %d\n\x00" as *const u8 as *const libc::c_char,
               *__errno_location());
        exit(1 as libc::c_int);
    };
}

pub unsafe fn db_close(mut table: *mut Table) {
    let mut pager: *mut Pager = (*table).pager;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*pager).num_pages {
        if !(*pager).pages[i as usize].is_null() {
            pager_flush(pager, i);
            free((*pager).pages[i as usize]);
            (*pager).pages[i as usize] = 0 as *mut libc::c_void
        }
        i = i.wrapping_add(1)
    }
    let mut result: libc::c_int = close((*pager).file_descriptor);
    if result == -(1 as libc::c_int) {
        println!(b"Error closing db file.\n\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < 100 as libc::c_int as libc::c_uint {
        let mut page: *mut libc::c_void = (*pager).pages[i_0 as usize];
        if !page.is_null() {
            free(page);
            (*pager).pages[i_0 as usize] = 0 as *mut libc::c_void
        }
        i_0 = i_0.wrapping_add(1)
    }
    free(pager as *mut libc::c_void);
    free(table as *mut libc::c_void);
}

pub unsafe fn do_meta_command(mut input_buffer: *mut InputBuffer,
                                         mut table: *mut Table)
 -> MetaCommandResult {
    if strcmp((*input_buffer).buffer,
              b".exit\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        close_input_buffer(input_buffer);
        db_close(table);
        exit(0 as libc::c_int);
    } else if strcmp((*input_buffer).buffer,
                     b".btree\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        println!(b"Tree:\n\x00" as *const u8 as *const libc::c_char);
        print_tree((*table).pager, 0 as libc::c_int as uint32_t,
                   0 as libc::c_int as uint32_t);
        return META_COMMAND_SUCCESS
    } else if strcmp((*input_buffer).buffer,
                     b".constants\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        println!(b"Constants:\n\x00" as *const u8 as *const libc::c_char);
        print_constants();
        return META_COMMAND_SUCCESS
    } else { return META_COMMAND_UNRECOGNIZED_COMMAND };
}

pub unsafe fn prepare_insert(mut input_buffer: *mut InputBuffer,
                                        mut statement: *mut Statement)
 -> PrepareResult {
    (*statement).type_0 = STATEMENT_INSERT;
    let mut keyword: *mut libc::c_char =
        strtok((*input_buffer).buffer,
               b" \x00" as *const u8 as *const libc::c_char);
    let mut id_string: *mut libc::c_char =
        strtok(0 as *mut libc::c_char,
               b" \x00" as *const u8 as *const libc::c_char);
    let mut username: *mut libc::c_char =
        strtok(0 as *mut libc::c_char,
               b" \x00" as *const u8 as *const libc::c_char);
    let mut email: *mut libc::c_char =
        strtok(0 as *mut libc::c_char,
               b" \x00" as *const u8 as *const libc::c_char);
    if id_string.is_null() || username.is_null() || email.is_null() {
        return PREPARE_SYNTAX_ERROR
    }
    let mut id: libc::c_int = atoi(id_string);
    if id < 0 as libc::c_int { return PREPARE_NEGATIVE_ID }
    if strlen(username) > 32 as libc::c_int as libc::c_ulong {
        return PREPARE_STRING_TOO_LONG
    }
    if strlen(email) > 255 as libc::c_int as libc::c_ulong {
        return PREPARE_STRING_TOO_LONG
    }
    (*statement).row_to_insert.id = id as uint32_t;
    strcpy((*statement).row_to_insert.username.as_mut_ptr(), username);
    strcpy((*statement).row_to_insert.email.as_mut_ptr(), email);
    return PREPARE_SUCCESS;
}

pub unsafe fn prepare_statement(mut input_buffer: *mut InputBuffer,
                                           mut statement: *mut Statement)
 -> PrepareResult {
    if strncmp((*input_buffer).buffer,
               b"insert\x00" as *const u8 as *const libc::c_char,
               6 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        return prepare_insert(input_buffer, statement)
    }
    if strcmp((*input_buffer).buffer,
              b"select\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        (*statement).type_0 = STATEMENT_SELECT;
        return PREPARE_SUCCESS
    }
    return PREPARE_UNRECOGNIZED_STATEMENT;
}
/*
Until we start recycling free pages, new pages will always
go onto the end of the database file
*/

pub unsafe fn get_unused_page_num(mut pager: *mut Pager)
 -> uint32_t {
    return (*pager).num_pages;
}

pub unsafe fn create_new_root(mut table: *mut Table,
                                         mut right_child_page_num: uint32_t) {
    /*
  Handle splitting the root.
  Old root copied to new page, becomes left child.
  Address of right child passed in.
  Re-initialize root page to contain the new root node.
  New root node points to two children.
  */
    let mut root: *mut libc::c_void =
        get_page((*table).pager, (*table).root_page_num);
    let mut right_child: *mut libc::c_void =
        get_page((*table).pager, right_child_page_num);
    let mut left_child_page_num: uint32_t =
        get_unused_page_num((*table).pager);
    let mut left_child: *mut libc::c_void =
        get_page((*table).pager, left_child_page_num);
    /* Left child has data copied from old root */
    memcpy(left_child, root, PAGE_SIZE as libc::c_ulong);
    set_node_root(left_child, 0 as libc::c_int != 0);
    /* Root node is a new internal node with one key and two children */
    initialize_internal_node(root);
    set_node_root(root, 1 as libc::c_int != 0);
    *internal_node_num_keys(root) = 1 as libc::c_int as uint32_t;
    *internal_node_child(root, 0 as libc::c_int as uint32_t) =
        left_child_page_num;
    let mut left_child_max_key: uint32_t = get_node_max_key(left_child);
    *internal_node_key(root, 0 as libc::c_int as uint32_t) =
        left_child_max_key;
    *internal_node_right_child(root) = right_child_page_num;
    *node_parent(left_child) = (*table).root_page_num;
    *node_parent(right_child) = (*table).root_page_num;
}

pub unsafe fn internal_node_insert(mut table: *mut Table,
                                              mut parent_page_num: uint32_t,
                                              mut child_page_num: uint32_t) {
    /*
  Add a new child/key pair to parent that corresponds to child
  */
    let mut parent: *mut libc::c_void =
        get_page((*table).pager, parent_page_num);
    let mut child: *mut libc::c_void =
        get_page((*table).pager, child_page_num);
    let mut child_max_key: uint32_t = get_node_max_key(child);
    let mut index: uint32_t = internal_node_find_child(parent, child_max_key);
    let mut original_num_keys: uint32_t = *internal_node_num_keys(parent);
    *internal_node_num_keys(parent) =
        original_num_keys.wrapping_add(1 as libc::c_int as libc::c_uint);
    if original_num_keys >= INTERNAL_NODE_MAX_CELLS {
        println!(b"Need to implement splitting internal node\n\x00" as *const u8
                   as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut right_child_page_num: uint32_t =
        *internal_node_right_child(parent);
    let mut right_child: *mut libc::c_void =
        get_page((*table).pager, right_child_page_num);
    if child_max_key > get_node_max_key(right_child) {
        /* Replace right child */
        *internal_node_child(parent, original_num_keys) =
            right_child_page_num;
        *internal_node_key(parent, original_num_keys) =
            get_node_max_key(right_child);
        *internal_node_right_child(parent) = child_page_num
    } else {
        /* Make room for the new cell */
        let mut i: uint32_t = original_num_keys;
        while i > index {
            let mut destination: *mut libc::c_void =
                internal_node_cell(parent, i) as *mut libc::c_void;
            let mut source: *mut libc::c_void =
                internal_node_cell(parent,
                                   i.wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint)) as
                    *mut libc::c_void;
            memcpy(destination, source,
                   INTERNAL_NODE_CELL_SIZE as libc::c_ulong);
            i = i.wrapping_sub(1)
        }
        *internal_node_child(parent, index) = child_page_num;
        *internal_node_key(parent, index) = child_max_key
    };
}

pub unsafe fn update_internal_node_key(mut node: *mut libc::c_void,
                                                  mut old_key: uint32_t,
                                                  mut new_key: uint32_t) {
    let mut old_child_index: uint32_t =
        internal_node_find_child(node, old_key);
    *internal_node_key(node, old_child_index) = new_key;
}

pub unsafe fn leaf_node_split_and_insert(mut cursor: *mut Cursor,
                                                    mut key: uint32_t,
                                                    mut value: *mut Row) {
    /*
  Create a new node and move half the cells over.
  Insert the new value in one of the two nodes.
  Update parent or create a new parent.
  */
    let mut old_node: *mut libc::c_void =
        get_page((*(*cursor).table).pager, (*cursor).page_num);
    let mut old_max: uint32_t = get_node_max_key(old_node);
    let mut new_page_num: uint32_t =
        get_unused_page_num((*(*cursor).table).pager);
    let mut new_node: *mut libc::c_void =
        get_page((*(*cursor).table).pager, new_page_num);
    initialize_leaf_node(new_node);
    *node_parent(new_node) = *node_parent(old_node);
    *leaf_node_next_leaf(new_node) = *leaf_node_next_leaf(old_node);
    *leaf_node_next_leaf(old_node) = new_page_num;
    /*
  All existing keys plus new key should should be divided
  evenly between old (left) and new (right) nodes.
  Starting from the right, move each key to correct position.
  */
    let mut i: int32_t = LEAF_NODE_MAX_CELLS as int32_t;
    while i >= 0 as libc::c_int {
        let mut destination_node: *mut libc::c_void = 0 as *mut libc::c_void;
        if i as libc::c_uint >= LEAF_NODE_LEFT_SPLIT_COUNT {
            destination_node = new_node
        } else { destination_node = old_node }
        let mut index_within_node: uint32_t =
            (i as libc::c_uint).wrapping_rem(LEAF_NODE_LEFT_SPLIT_COUNT);
        let mut destination: *mut libc::c_void =
            leaf_node_cell(destination_node, index_within_node);
        if i as libc::c_uint == (*cursor).cell_num {
            serialize_row(value,
                          leaf_node_value(destination_node,
                                          index_within_node));
            *leaf_node_key(destination_node, index_within_node) = key
        } else if i as libc::c_uint > (*cursor).cell_num {
            memcpy(destination,
                   leaf_node_cell(old_node,
                                  (i - 1 as libc::c_int) as uint32_t),
                   LEAF_NODE_CELL_SIZE as libc::c_ulong);
        } else {
            memcpy(destination, leaf_node_cell(old_node, i as uint32_t),
                   LEAF_NODE_CELL_SIZE as libc::c_ulong);
        }
        i -= 1
    }
    /* Update cell count on both leaf nodes */
    *leaf_node_num_cells(old_node) = LEAF_NODE_LEFT_SPLIT_COUNT;
    *leaf_node_num_cells(new_node) = LEAF_NODE_RIGHT_SPLIT_COUNT;
    if is_node_root(old_node) {
        return create_new_root((*cursor).table, new_page_num)
    } else {
        let mut parent_page_num: uint32_t = *node_parent(old_node);
        let mut new_max: uint32_t = get_node_max_key(old_node);
        let mut parent: *mut libc::c_void =
            get_page((*(*cursor).table).pager, parent_page_num);
        update_internal_node_key(parent, old_max, new_max);
        internal_node_insert((*cursor).table, parent_page_num, new_page_num);
        return
    };
}

pub unsafe fn leaf_node_insert(mut cursor: *mut Cursor,
                                          mut key: uint32_t,
                                          mut value: *mut Row) {
    let mut node: *mut libc::c_void =
        get_page((*(*cursor).table).pager, (*cursor).page_num);
    let mut num_cells: uint32_t = *leaf_node_num_cells(node);
    if num_cells >= LEAF_NODE_MAX_CELLS {
        // Node full
        leaf_node_split_and_insert(cursor, key, value);
        return
    }
    if (*cursor).cell_num < num_cells {
        // Make room for new cell
        let mut i: uint32_t = num_cells;
        while i > (*cursor).cell_num {
            memcpy(leaf_node_cell(node, i),
                   leaf_node_cell(node,
                                  i.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint)),
                   LEAF_NODE_CELL_SIZE as libc::c_ulong);
            i = i.wrapping_sub(1)
        }
    }
    let ref mut fresh0 = *leaf_node_num_cells(node);
    *fresh0 =
        (*fresh0 as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    *leaf_node_key(node, (*cursor).cell_num) = key;
    serialize_row(value, leaf_node_value(node, (*cursor).cell_num));
}

pub unsafe fn execute_insert(mut statement: *mut Statement,
                                        mut table: *mut Table)
 -> ExecuteResult {
    let mut row_to_insert: *mut Row = &mut (*statement).row_to_insert;
    let mut key_to_insert: uint32_t = (*row_to_insert).id;
    let mut cursor: *mut Cursor = table_find(table, key_to_insert);
    let mut node: *mut libc::c_void =
        get_page((*table).pager, (*cursor).page_num);
    let mut num_cells: uint32_t = *leaf_node_num_cells(node);
    if (*cursor).cell_num < num_cells {
        let mut key_at_index: uint32_t =
            *leaf_node_key(node, (*cursor).cell_num);
        if key_at_index == key_to_insert { return EXECUTE_DUPLICATE_KEY }
    }
    leaf_node_insert(cursor, (*row_to_insert).id, row_to_insert);
    free(cursor as *mut libc::c_void);
    return EXECUTE_SUCCESS;
}

pub unsafe fn execute_select(mut statement: *mut Statement,
                                        mut table: *mut Table)
 -> ExecuteResult {
    let mut cursor: *mut Cursor = table_start(table);
    let mut row: Row = Row{id: 0, username: [0; 33], email: [0; 256],};
    while !(*cursor).end_of_table {
        deserialize_row(cursor_value(cursor), &mut row);
        print_row(&mut row);
        cursor_advance(cursor);
    }
    free(cursor as *mut libc::c_void);
    return EXECUTE_SUCCESS;
}

pub unsafe fn execute_statement(mut statement: *mut Statement,
                                           mut table: *mut Table)
 -> ExecuteResult {
    match (*statement).type_0 as libc::c_uint {
        0 => { return execute_insert(statement, table) }
        1 => { return execute_select(statement, table) }
        _ => { }
    }
    panic!("Reached end of non-void function without returning");
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    if argc < 2 as libc::c_int {
        println!(b"Must supply a database filename.\n\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut filename: *mut libc::c_char =
        *argv.offset(1 as libc::c_int as isize);
    let mut table: *mut Table = db_open(filename);
    let mut input_buffer: *mut InputBuffer = new_input_buffer();
    loop  {
        print_prompt();
        read_input(input_buffer);
        if *(*input_buffer).buffer.offset(0 as libc::c_int as isize) as
               libc::c_int == '.' as i32 {
            match do_meta_command(input_buffer, table) as libc::c_uint {
                0 => { continue ; }
                1 => {
                    println!(b"Unrecognized command \'%s\'\n\x00" as *const u8
                               as *const libc::c_char,
                           (*input_buffer).buffer);
                    continue ;
                }
                _ => { }
            }
        }
        let mut statement: Statement =
            Statement{type_0: STATEMENT_INSERT,
                      row_to_insert:
                          Row{id: 0, username: [0; 33], email: [0; 256],},};
        match prepare_statement(input_buffer, &mut statement) as libc::c_uint
            {
            1 => {
                println!(b"ID must be positive.\n\x00" as *const u8 as
                           *const libc::c_char);
            }
            2 => {
                println!(b"String is too long.\n\x00" as *const u8 as
                           *const libc::c_char);
            }
            3 => {
                println!(b"Syntax error. Could not parse statement.\n\x00" as
                           *const u8 as *const libc::c_char);
            }
            4 => {
                println!(b"Unrecognized keyword at start of \'%s\'.\n\x00" as
                           *const u8 as *const libc::c_char,
                       (*input_buffer).buffer);
            }
            0 | _ => {
                match execute_statement(&mut statement, table) as libc::c_uint
                    {
                    0 => {
                        println!(b"Executed.\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    1 => {
                        println!(b"Error: Duplicate key.\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    _ => { }
                }
            }
        }
    };
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    ID_SIZE = ::std::mem::size_of::<uint32_t>() as libc::c_ulong as uint32_t;
    USERNAME_SIZE =
        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as
            uint32_t;
    EMAIL_SIZE =
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as
            uint32_t;
    USERNAME_OFFSET = ID_OFFSET.wrapping_add(ID_SIZE);
    EMAIL_OFFSET = USERNAME_OFFSET.wrapping_add(USERNAME_SIZE);
    ROW_SIZE = ID_SIZE.wrapping_add(USERNAME_SIZE).wrapping_add(EMAIL_SIZE);
    PARENT_POINTER_OFFSET = IS_ROOT_OFFSET.wrapping_add(IS_ROOT_SIZE);
    COMMON_NODE_HEADER_SIZE =
        NODE_TYPE_SIZE.wrapping_add(IS_ROOT_SIZE).wrapping_add(PARENT_POINTER_SIZE)
            as uint8_t;
    INTERNAL_NODE_RIGHT_CHILD_OFFSET =
        INTERNAL_NODE_NUM_KEYS_OFFSET.wrapping_add(INTERNAL_NODE_NUM_KEYS_SIZE);
    INTERNAL_NODE_HEADER_SIZE =
        (COMMON_NODE_HEADER_SIZE as
             libc::c_uint).wrapping_add(INTERNAL_NODE_NUM_KEYS_SIZE).wrapping_add(INTERNAL_NODE_RIGHT_CHILD_SIZE);
    INTERNAL_NODE_CELL_SIZE =
        INTERNAL_NODE_CHILD_SIZE.wrapping_add(INTERNAL_NODE_KEY_SIZE);
    LEAF_NODE_NEXT_LEAF_OFFSET =
        LEAF_NODE_NUM_CELLS_OFFSET.wrapping_add(LEAF_NODE_NUM_CELLS_SIZE);
    LEAF_NODE_HEADER_SIZE =
        (COMMON_NODE_HEADER_SIZE as
             libc::c_uint).wrapping_add(LEAF_NODE_NUM_CELLS_SIZE).wrapping_add(LEAF_NODE_NEXT_LEAF_SIZE);
    LEAF_NODE_VALUE_OFFSET =
        LEAF_NODE_KEY_OFFSET.wrapping_add(LEAF_NODE_KEY_SIZE);
    LEAF_NODE_CELL_SIZE =
        LEAF_NODE_KEY_SIZE.wrapping_add(LEAF_NODE_VALUE_SIZE);
    LEAF_NODE_SPACE_FOR_CELLS = PAGE_SIZE.wrapping_sub(LEAF_NODE_HEADER_SIZE);
    LEAF_NODE_MAX_CELLS =
        LEAF_NODE_SPACE_FOR_CELLS.wrapping_div(LEAF_NODE_CELL_SIZE);
    LEAF_NODE_RIGHT_SPLIT_COUNT =
        LEAF_NODE_MAX_CELLS.wrapping_add(1 as libc::c_int as
                                             libc::c_uint).wrapping_div(2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
    LEAF_NODE_LEFT_SPLIT_COUNT =
        LEAF_NODE_MAX_CELLS.wrapping_add(1 as libc::c_int as
                                             libc::c_uint).wrapping_sub(LEAF_NODE_RIGHT_SPLIT_COUNT)
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

fn main() {
    println!("Hello World!");
}