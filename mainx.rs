// ############################################################################################################################################
// 
// Lists.rs
// 
// Copyright :      Copyright Free. Paul made it though.
// License :        Copy and use with or without charge whenever you like for any purpose.
// 
// Description :   
//                      1. Start with separate data in 5 variables.
//                      2. Merge into a 'list'.
//                      3. Count items in list and Read data 2 and 4 back out into 1 new variables and a print statement.
//                      4. Put list into a new variable as a string.
//                      5. Count items in the string, Try to read data 3 and 5 from string into 2 new variables. (expect fail).
//                      6. Convert string to list.
//                      7. Count items in the string, Try to read data 3 and 5 from converted into 2 new variables. (expect success).
// 
//                  * Document success and fail in boiler plate notes.
//                  Repeat in RUST.
// 
// Status :         10 - Live code trial.
// 
// Version History :        
//                  v1.00 PME 2019/07/13 08:49 - Python Version : Full code written and tested in 1 hr.
//                  v1.01 PME 2019/07/15 10:07 - Rust Conversion: Started.
//                  v1.02 PME 2019/07/15 10:31 - Converted print statements, comments, added ; to line end. Added Lets to variable creation.
//                   
//
// Useful Python References:
//                  https:// www.lucidar.me/en/python/count-elements/
//                  https:// www.programiz.com/python-programming/list
//                  https:// stackoverflow.com/questions/17796446/convert-a-list-to-a-string-and-back
//                  https:// docs.python.org/3/library/ast.html
// 
// Useful Rust References:
//                  Getting started: https:// doc.rust-lang.org/book/ch01-02-hello-world.html
//                  Variables: https:// doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
// Key notes
//                  Aside from mild language and code presentation changes, variables tend to follow the 'FORTH' model,
//                  using the 'stack' and 'heap' model.
//

fn main() {

    println!("\n\n\n\n *******************************************************************");
    println!("\n                 Understanding Rust Variables "); 
    println!("\n *******************************************************************");
    println!(" Useful notes:");
    println!(" When counting, we start from 0, not 1.");
    println!(" See source code for useful references.");

    // ################################################################################################
    // 
    //  1. Start with separate data in 5 variables.
    // 
    //  v1.00 PME 2019/07/13 08:49 - creation (Python) (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 1.1. Creating list items. "); 

    let list_item_1 = "first";
    let list_item_2 = "second";
    let list_item_3 = "third";
    let list_item_4 = "fourth";
    let list_item_5 = "fifth";


    println!(" 1.2. List items are : {} ", list_item_1, list_item_2, list_item_3, list_item_4, list_item_5);



    // ################################################################################################
    // 
    // 2. Merge into a 'list'.
    // 
    // v1.00 PME 2019/07/13 09:02 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 2.1. Merge into a 'list'. "); 
    list_true = [list_item_1, list_item_2, list_item_3, list_item_4, list_item_5];


    println!(" 2.2. Print List:");
    println!(list_true);



    // ################################################################################################
    // 
    // 3. Count items in list and Read data 2 and 4 back out into 2 new variables.
    // 
    // v1.00 PME 2019/07/13 09:02 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 3 Count items in list and Read data 2 and 4 back out into 1 new variables and a print statement. "); 


    println!(" 3.1.1 Count items in list");
    items_in_list_true = len(list_true);
    println!(" 3.1.2 Number of items in list are: ", items_in_list_true);


    println!(" 3.2. Put a copy of list item 2 into variable_1");
    variable_1 = (list_true[1]);
    println!(variable_1);


    println!(" 2.3. Print item 4:");
    println!(list_true[3]);




    // ################################################################################################
    // 
    // 4. Put list into a new variable as a string.
    // 
    // v1.00 PME 2019/07/13 09:02 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 4.1 Put list into a new variable as a string.. "); 
    list_string = str(list_true);


    println!(" 4.2 Print contents of new string based list variable. "); 
    println!(list_string);



    // ################################################################################################
    // 
    // 5. Try to read data 3 and 5 from string into 2 new variables. (expect fail).
    // 
    // v1.00 PME 2019/07/13 09:20 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 5. Try to read data 3 and 5 from string into 1 new variables and a print. (expect fail). "); 


    println!(" 5.1.1 Count items in list");
    items_in_list_string = len(list_string);
    println!(" 5.1.2 Number of items in list are: ", items_in_list_string)


    println!(" 5.2. Put item 3 into variable_2.");
    variable_2 = (list_string[2]);
    println!(variable_2);


    println!(" 5.4. Print item 5:");
    println!(list_string[4]);


    println!(" 5.5. Describe the error:");
    println!(" Rather than printing list item, it prints the character from the string relating to position.");



    // ################################################################################################
    // 
    // 6. Convert string to list.
    // 
    // v1.00 PME 2019/07/13 09:36 - creation (Python)
    // 

    // NOTE: I couldn't seem to get JSON functions to work here. Stage 7 still treated the converted list as a string.
    //       However, literal_eval resolved the issue as the resulting data was presented to Python as a list.
    //       I've kept JSON code as an example for later review. I would function it, but that would add a level of complication to
    //       this 'simple' python code.


    // Useful Reference:
    // Convert list to string:   string = json.dumps(list)
    // Convert string to list:   list = json.loads(string)

    println!("\n -------------------------------------");
    println!(" 6. Convert string to list.");
    println!(" There doesn't seem to be a way to do this in a simple Python.  Most references point to literal_eval method... \n\n");

    println!(" 6.1. Import literal_eval module.");
    from ast import literal_eval;

    println!(" 6.2. Convert string containing a text version of the list back into a real list.");
    converted_back_into_list = literal_eval(list_string);

    println!(" 6.3. Show contents of list_string variable before conversion:");
    println!(list_string);

    println!(" 6.4 Show contents of string_converted_back_into_list variable after conversion.");
    println!(converted_back_into_list);



    // ################################################################################################
    // 
    // 7. Try to read data 3 and 5 from converted into 2 new variables. (expect success).
    // 
    // v1.00 PME 2019/07/13 09:52 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 7. Try to read data 3 and 5 from converted into 2 new variables. (expect success). "); 


    println!(" 7.1.1 Count items in list");
    items_in_converted_list = len(converted_back_into_list);
    println!(" 7.1.2 Number of items in list are: ", items_in_converted_list);

    println!(" 7.2. Put a copy of list item 3 into variable_1");
    variable_3 = (converted_back_into_list[2]);
    println!(variable_3);


    println!(" 7.3. Print item 5:");
    println!(converted_back_into_list[4]);


    println!("\n -------------------------------------");
    println!("\n\n END. \n\n\n\n");




    // ################################################################################################
    // 
    // EXTRA LIST FUN....


    // ################################################################################################
    // 
    // 8. Test the list
    // 
    // v1.00 PME 2019/07/13 18:12 - creation (Python)
    // 

    println!("8. Is 'fourth' in the list?");
    println!("fourth" in converted_back_into_list);



    // ################################################################################################
    // 
    // 9. "for" loops
    // 
    // v1.00 PME 2019/07/13 18:12 - creation (Python)
    // 

    println!("\n 9. What's in the list?");
    for items in converted_back_into_list:
        println!(" Items:", items);

    // ################################################################################################
    // 
    // 10. "for" Count occurances of 'first' in list."
    // 
    // v1.00 PME 2019/07/13 18:24 - creation (Python)
    // 

    println!("\n 10. Count occurances of 'first' in list.");
    count_firsts = converted_back_into_list.count('first');
    println!(count_firsts);




    // ################################################################################################
    // 
    // 11. Copy the list, leaving a copy on the original version.
    // 
    // v1.00 PME 2019/07/13 18:45 - creation (Python)
    // 

    println!("\n 11. Copy the list to a new list (not string...)");
    new_list = converted_back_into_list[:];
    println!(new_list);

    // ################################################################################################
    // 
    // 12. Reverse the list.
    // 
    // v1.00 PME 2019/07/13 18:42 - creation (Python)
    // 

    println!("\n 12. Reverse the list.");
    new_list.reverse();
    println!(new_list);

// End function : main
}