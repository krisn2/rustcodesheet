
pub fn data_types() {

    //+-----------------------------------------------+
    //|           Note: variable are mutable          |
    //|              by default in rust               |
    //|         for make variable mutable use `mut`   |
    //+-----------------------------------------------+
    
    //****syntax**//
    //  binding variablename : type = value
    
    //Unit type
        let x: () = (); // unit type (it's means non value) or (void)
        println!("Unit type: {:?}", x);
    
    //Interger type 
        // |-------------------|
        // |len|Signed|Unsigned|
        // |-------------------|
        // | 8 |  i8  |  u8  |
        // | 16|  i16 |  u16 |
        // | 32|  i32 |  u32 |
        // | 64|  i64 |  u64 |
        // |-------------------|
    
        //Signed interger contains negative and positive both value .
        //Unsigned interger contains only positive value.
    
        let inti8: i8 = -128;
        let intu8: u8 = 255;
        let inti16: i16 = -32768;
        let intu16: u16 = 65535;
        let inti32: i32 = -2147483648;
        let intu32: u32 = 4294967295;
        let inti64: i64 = -9223372036854775808;
        let intu64: u64 = 18446744073709551615;
    
        println!("Interger type: {:?}", (inti8, intu8, inti16, intu16, inti32, intu32, inti64, intu64));
    
    //Float type
        let floatf32: f32 = 3.14;
        let floatf64: f64 = 3.14;
        println!("Float type: {:?}", (floatf32, floatf64));
    
    //Boolean type
        let boolture: bool = true;
        let boolfalse:bool = false;
        println!("Boolean type: {:?}", (boolture, boolfalse));
    
    //Char type
        let data_type_char: char = 'a';
        println!("Char type: {}", data_type_char);
    
    
    //String type
        let mut first_name: &str  = "krishn";
        println!("String type using mut keyword: {}  ", first_name);
    
        first_name = "harsh";
    println!("String type: {}  ", first_name);
    
    
    //Tuple type
        let tup: (i32, f64, &str) = (200, 3.14, "tuple");
        println!("Tuple type: {:?}", tup);
    
    
    //Array type
        let arr: [i32; 5] = [1, -2, 3, 4, 5];
        println!("Singed Array type: {:?}", arr);
    
    }
    
    
    