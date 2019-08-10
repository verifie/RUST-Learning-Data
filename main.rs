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
//                  Printing (lists etc): https://doc.rust-lang.org/std/fmt/
// Key notes
//                  Aside from mild language and code presentation changes, variables tend to follow the 'FORTH' model,
//                  using the 'stack' and 'heap' model.
//


fn main() {

 //   loop {
 //       println!("Paul's variable fun...");
        primary_function_repeater();
 //   }

// End main()
}



// Here's the repeat function.
fn primary_function_repeater() {

    use std::io::{stdin,stdout,Write};

    let mut user_repeat_requests=String::new();
    print!("  How many times should we do this? ");

    let _=stdout().flush();

    stdin().read_line(&mut user_repeat_requests).expect("  Did not enter a correct string");
    if let Some('\n') = user_repeat_requests.chars().next_back() {
        user_repeat_requests.pop();
    }

    if let Some('\r') = user_repeat_requests.chars().next_back() {
        user_repeat_requests.pop();
    }

    println!("  You typed: {}", user_repeat_requests);

    let count_repeats: u32 = user_repeat_requests.parse().unwrap();
    


    // Reference:
    // https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
    let mut count = 0u32;

    println!("  Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("        -------> three!!!!!1");

            // Skip the rest of this iteration
            continue;
        }

        // Execute things we want to do in the loop.
        do_tricks_with_variables();

        // Show the current count
        println!("{}", count);

        // Check to see if we've reached the number of repeats the user asked for.
        if count == count_repeats {

            // If we get here, we have reached the number of repeats the user asked for.
            println!("   We reached the number of repeats the user asked for. Ending Loop");

            // Exit this loop
            break;
        }
    }
}

// Here's the main program function.
fn do_tricks_with_variables() {

    println!("\n\n\n\n *******************************************************************");
    println!("\n                 Understanding Rust Variables "); 
    println!("\n *******************************************************************");
    println!(" Useful notes:");
    println!(" When counting, we start from 0, not 1.");
    println!(" See source code for useful references.");
    println!(" \n Here goes.... \n\n");

    // ################################################################################################
    // 
    // 1. Start with separate data in 5 variables.
    // 
    // v1.00 PME 2019/07/13 08:49 - creation (Python) (Python)
    // v1.01 PME 2019/07/15 11:50 - Rust version with comment differences.
    // 

    println!("\n -------------------------------------");
    println!("\n 1.1. Creating list items. "); 

    let list_item_1 = "first";
    let list_item_2 = "second";
    let list_item_3 = "third";
    let list_item_4 = "fourth";
    let list_item_5 = "fifth";
    let list_item_6 = "first";

    // Key differences to Python: You must use {} in the string to place the following variable references (in order). Similar to enforced
    // use of %s in Python, except you have to do it.
    println!(" 1.2. List items are : {} {} {} {} {} {} ", list_item_1, list_item_2, list_item_3, list_item_4, list_item_5, list_item_6);


    // ################################################################################################
    // 
    // 2. Merge into a 'list'.
    // 
    // v1.00 PME 2019/07/13 09:02 - creation (Python)
    // v1.01 PME 2019/07/15 11:58 - Rust version with comment differences.
    // 

    println!("\n -------------------------------------");
    println!("\n 2. Put the items into an Array (or list). "); 
    let list_true = vec![list_item_1, list_item_2, list_item_3, list_item_4, list_item_5, list_item_6];          // Added "let" to setup the variable in Rust. Also added vec! to instruct Rust. learn more about vec!


    println!("\n 2.2. Print the list:");
    println!("      {:?}", list_true);                              // Modified to include print variable in string requirement 


    // ################################################################################################
    // 
    // 3. Count items in list and Read data 2 and 4 back out into 2 new variables.
    // 
    // v1.00 PME 2019/07/13 09:02 - creation (Python)
    // v1.01 PME 2019/07/15 12:43 - Rust version with comment differences. BUG not correct yet - string without separators presented.
    // 
    // Reference
    // Understanding iterators https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html

    println!("\n -------------------------------------");
    println!("\n 3 Count items in list and Read data 2 and 4 back out into 1 new variables and a print statement. \n"); 


    println!(" 3.1.1 Count items in list");
    let items_in_list_true = list_true.len();                                       // Instead of len(variable), its variable.len()
    println!(" 3.1.2 Number of items in list are: {:?}", items_in_list_true);


    println!("\n 3.2. Put a copy of list item 2 into variable_1");
    let variable_1 = list_true[1];
    println!("      Variable 1 now contains:  {}", variable_1);


    println!("\n 2.3. Print item 4:");
    println!("      Printing out item 4 from the list: {:?}", list_true[3]);





    // ################################################################################################
    // 
    // 4. Put list into a new variable as a string.
    // 
    // v1.00 PME 2019/07/13 09:02 - creation (Python)
    // v1.01 PME 2019/07/15 24:05 - Conversion to Rust. More challenging than Python - you have to instruct more.
    //
    // References:
    // https://stackoverflow.com/questions/28311868/what-is-the-equivalent-of-the-join-operator-over-a-vector-of-strings


    // THINGS THAT DON'T WORK FOR THIS:
    // These all fail to send the data as a text string version of the array with " and ,. They are just end-to-end
    // e.g.firstsecondthirdfourthfifth, not ["first", "second"] ... etc:
    //let list_string = &String::from(list_true[4]);                            // Not used in this, but useful reference. Picking 1 item from an array (item 5(4))
    //let list_string: String = list_true.into_iter().map(|i| i.to_string()).collect::<String>();       // Original worked.
    //let list_string: String = list_true.into_iter().collect::<String>();                                // Also works... appears the same as prior line.
    //let list_string: String = ToString::to_string(&list_true);                // This just fails.
    //let list_string: String = list_true.iter().map(|i| i.to_string()).collect(); 
    // let list_string: String = list_true.iter().join(&0); 
    //let list_string: String = ToString::to_string(&list_true);                // This just fails.
    //assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
    //println!("{}", list_string);                                              // Note, as this is a string copy of the list, we dont {:?} in print.
    //let string_list = vec!["Foo".to_string(),"Bar".to_string()];



    println!("\n -------------------------------------");
    println!("\n 4.1 Put list into a new variable as a string.. \n"); 

    println!(" 4.2 Print contents of new string based list variable. "); 

    // Unlike Python, we need to tell Rust that a string is mutable, meaning changable (it's automatically immutable). "let mut" sets this up for us.
    // Also, unlike Python, when we put a list (array) into a string, Rust doesn't include the separators, and pops them all together, one after
    // another.  E.g. ["one", "two"] becomes onetwo.  We can manually re-construct a string to mirror the appearance of the array.
    // However, to identify the contents of the array in Rust, we will need to drop the [ ] space " characters and use the , as a delimiter.

    let mut list_string = String::from("[\u{22}");                              // Start a mutable string, append [" to the start.  \u{22}  is "
    list_string.push_str(&list_true.join("\u{22}, \u{22}"));                    // Now add the contents of the array, separating with ", "
    list_string.push_str("\u{22}]");                                            // End the string with "]

    println!("      {:?}", list_true);                                                // Print the original array.
    println!("      {}", list_string);                                                // Print the new string



    // ################################################################################################
    // 
    // 5. Try to read data 3 and 5 from string into 2 new variables. (expect fail).
    // 
    // v1.00 PME 2019/07/13 09:20 - creation (Python)
    // v1.01 PME 2019/07/15 24:21 - Rust version with comment differences.
    // 

    println!("\n -------------------------------------");
    println!("\n 5. Try to read data 3 and 5 from string into 1 new variables and a print. (expect fail). \n"); 


    println!(" 5.1.1 Count items in list");
    let items_in_list_string = list_string.len();

    println!(" 5.1.2 Print the variable containing the count of items in list : {}", items_in_list_string);

    println!("\n ** Note you can't lookup characters in a string using the same method as items in a list.");

    // Pulling characters from a string requires a different process from an array.

    // println!(" 5.2. Put item 3 into variable_2.");
    //let variable_2 = list_string[2];
    //println!("{}", variable_2);

    //println!(" 5.4. Print item 5:");
    //println!("{}", &list_string[4]);

    //println!(" 5.5. Describe the error:");
    //println!(" Rather than printing list item, it prints the character from the string relating to position.");




    // ################################################################################################
    // 
    // 6. Convert string to list.
    // 
    // v1.00 PME 2019/07/13 09:36 - creation (Python)
    // v1.01 PME 2019/07/18 22:01 - Rust conversion.
    // 

    // Python Note.
    // NOTE: I couldn't seem to get JSON functions to work here. Stage 7 still treated the converted list as a string.
    //       However, literal_eval resolved the issue as the resulting data was presented to Python as a list.
    //       Once other characters are removed, let converted_back_into_list: Vec<&str> = cleaned_list_string.split(",").collect();
    //       is the Rust equiv.

    // Useful Reference for Python equiv:
    // Convert list to string:   string = json.dumps(list)
    // Convert string to list:   list = json.loads(string)

    // First, we strip out the characters, popping the result into a new variable.  We don't need to do this in Python, as it
    // picks out the data and automagically discards the pretty list separation characters.
    // Note, we use a function here to do this.
    // Reference : https://www.rosettacode.org/wiki/Strip_a_set_of_characters_from_a_string#Rust

    println!("\n -------------------------------------");
    println!("\n 6. Convert string to list. \n");

    // Strip out the characters.
    println!(" 6.1. Strip out the characters added to text string to make it human readable : []\u{22} .");
    let cleaned_list_string = strip_characters(&list_string, "[]\u{22} ");
    println!("      {}", cleaned_list_string);


    // Next, we take the resulting text string and split it into an array of items, everytime we see the character in-between .split("char")
    println!("\n 6.2. Convert string containing a text version of the list back into a real list.");
    let converted_back_into_list: Vec<&str> = cleaned_list_string.split(",").collect();


    // Now we just print out the information.  SHowing before list conversion, and after.
    println!("\n 6.3. Show contents of list_string variable before conversion:");
    println!("      {}", list_string);

    println!("\n 6.4 Show contents of string_converted_back_into_list variable after conversion.");
    println!("      {:?}", converted_back_into_list);



    // ################################################################################################
    // 
    // 7. Try to read data 3 and 5 from converted into 2 new variables. (expect success).
    // 
    // v1.00 PME 2019/07/13 09:52 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 7. Try to read data 3 and 5 from converted into 2 new variables. (expect success). \n"); 


    println!(" 7.1.1 Count items in list");
    let items_in_converted_list = converted_back_into_list.len();

    println!("7.1.2 Number of items in list are: {}", items_in_converted_list);


    println!("\n 7.2. Put a copy of list item 3 into variable_1");
    let variable_3 = converted_back_into_list[2];
    println!("       {}", variable_3);


    println!("\n 7.3. Print item 5:");
    println!("       {}", converted_back_into_list[4]);                                    // We pick list item 4, which is human 5 taking into account 0.





    // ################################################################################################
    // 
    // EXTRA LIST FUN....


    // ################################################################################################
    // 
    // 8. Test the list
    // 
    // v1.00 PME 2019/07/13 18:12 - creation (Python)
    // v1.01 PME 2019/07/18 22:10 - Rust conversion, started.
    // v1.02 PME 2019/07/18 23:27 - Rust version, with extended function learning.
    //
    // References: 
    // https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html

    // Stupid things I tried first, guessing...
    //converted_back_into_list.find("fourth")                               // << This failed as .find isn't a thing.
    //find_string_content(&converted_back_into_list: &str, "fourth");       // << This failed as it isn't formed correctly.
    //if converted_back_into_list.words().any(|x| x == "fourth") {          // << I dont get the construction. Must learn!
    //    println!("Found it!");
    //}


    println!("\n -------------------------------------");
    println!("\n 8.1 Is 'fourth' in the list? \n");

    // 8.1.1 - Set a search variable.
    let search_for_this = "fourth";                                                     // Set a variable with the term we wish to search.

    // Firstly, we'll do it in straight code. Then we'll move it to a function. I discovered this causes move issues and so we must copy
    // the data to inspect it.

    // 8.1.2 - Use an "IF" statement with ".contains" to test the array (list).
    if converted_back_into_list.contains(&search_for_this) {
        println!("     Found it! \n");
        } else {
            println!("      We didn't find {}. Sorry! \n",  search_for_this);
        }


    // We want to prove the search works even if the searched item isn't in the list.
    println!("\n 8.2 Is 'First', 'Sixth' and 'Seventh' in the list? (Using a function) \n");

    let look_at_this = &converted_back_into_list;                                       // We must copy the data to inspect it.
    
    // The 1st search looks for First, with capitalisation that doesn't match the data in our array. We expect it to fail.
    let search_for_this = "First";                                                      // Set a variable with the term we wish to search.
    find_text_in_an_array(look_at_this.to_vec(), &search_for_this);                     // Call our search function with source and search data.
    
    // Now we search for the correct title case version. It will work!
    let search_for_this = "first";                                                      // Set a variable with the term we wish to search.
    find_text_in_an_array(look_at_this.to_vec(), &search_for_this);                     // Call our search function with source and search data.

    // Now we look for something we know isn't there.
    let search_for_this = "Sixth";                                                      // Set a variable with the term we wish to search.
    find_text_in_an_array(look_at_this.to_vec(), &search_for_this);                     // Call our search function with source and search data.

    // And again, we look for something we know isn't there.
    let search_for_this = "Seventh";                                                    // Set a variable with the term we wish to search.
    find_text_in_an_array(look_at_this.to_vec(), search_for_this);                      // Call our search function with source and search data.




    // ################################################################################################
    // 
    // 9. "for" loops
    // 
    // v1.00 PME 2019/07/13 18:12 - creation (Python)
    // v1.01 PME 2019/07/18 23:43 - Rust conversion, started.
    // v1.02 PME 2019/07/18 23:52 - Rust conversion, Finished
    // 

    println!("\n -------------------------------------");
    println!("\n 9. What's in the list? \n");

    let count_firsts = &converted_back_into_list;                                       // We must copy the data to inspect it.
    
    for items in count_firsts {                                             // For loop, based on items in array.
        println!("     Items: {}", items);
        }




    // ################################################################################################
    // 
    // 10. "for" Count occurances of 'first' in list."
    // 
    // v1.00 PME 2019/07/13 18:24 - creation (Python)
    // v1.01 PME 2019/07/18 23:52 - Rust conversion, Started.
    // v1.02 PME 2019/07/19 00:21 - Rust conversion, Finished
    //
    // Reference:
    // https://stackoverflow.com/questions/45353757/how-to-count-the-elements-in-a-vector-with-some-value-without-looping/45353922
    
    println!("\n -------------------------------------");
    println!("\n 10. Count occurances of 'first' in list. \n");

    let count_this = &converted_back_into_list;                                       // We must copy the data to inspect it.
    
    // As per the reference example.
    let look_for = "first";
    println!("     There are {} {}s in the list.", count_this.iter().filter(|&n| *n == look_for).count(), look_for);

    // A bit easier to read this version. The action is separated from the print statement.
    let look_for = "eighth";
    let searches_found = count_this.iter().filter(|&n| *n == look_for).count();
    println!("     There are {} {}s in the list.", searches_found, look_for);



    // ################################################################################################
    // 
    // 11. Copy the list, leaving a copy on the original version.
    // 
    // v1.00 PME 2019/07/13 18:45 - creation (Python)
    // v1.01 PME 2019/07/18 00:22 - Rust conversion, Started.
    // v1.02 PME 2019/07/19 00:24 - Rust conversion, Finished
    // 

    // Very similar to Python.  Just add a let and a ;. No need to define it's a list.

    println!("\n -------------------------------------");
    println!("\n 11. Copy the list to a new list (not string...) \n");

    let mut new_list = converted_back_into_list;
    println!("     {:?}", new_list);




    // ################################################################################################
    // 
    // 12. Reverse the list.
    // 
    // v1.00 PME 2019/07/13 18:42 - creation (Python)
    // 

    println!("\n -------------------------------------");
    println!("\n 12.1 Reverse the list.");

    // To reverse the list, it must be mutable.  We simply reverse the list by adding .reverse() to the array name.
    new_list.reverse();

    println!(" The reversed list now reads : {:?}", new_list);

    
    println!("\n 12.2 Reverse the list back.");

    new_list.reverse();

    println!(" The reversed list now reads : {:?}", new_list);





    // ################################################################################################
    // 
    // 13. Reverse the order of characters of a new string.
    // 
    // v0.01 PME 2019/07/19 22:51 - Creatioon first in rust
    //
    // References:
    // https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust

    println!("\n -------------------------------------");
    println!("\n 13. Reverse the order of characters of a new string. \n");
    
    // Setup the string to reverse. Print the original string content to screen.
    let reverse_this_string = "!olleh";
    println!(" 13.1 String to reverse: {}", reverse_this_string);

    // Reverse the characters in the new string.
    // let reversed_string = reverse_this_string.chars().rev();             // This doesnt work....
    let reversed_string = reverse_this_string.chars().rev().collect::<String>();

    // Print the reversed string to the screen
    println!(" 13.2 String reversed:   {}", reversed_string);




    // ################################################################################################
    // 
    // 14. Reverse the order of characters of an item in the list.
    // 
    // v0.01 PME 2019/07/19 22:51 - Creatioon first in rust

    println!("\n -------------------------------------");
    println!("\n 14. Reverse the order of characters of the 3rd item in the list. \n");

    // Print the list and the 3rd item in the list    
    println!("\n The list contains: {:?}", new_list);
    println!(" The third item in the list is :  {}", new_list[2]);

    // Reverse the 3rd item.
    let reversed_item = new_list[2]
        .chars()
        .rev()
        .collect::<String>();
    

    // Now print the list with the 3rd item reversed.
    println!("\n The reversed list item contains :  {}", reversed_item);

    // The list contains:
    println!("\n The list initially contains :                      {:?}", new_list);

    // Now put the data in the variable back into the list.  As the array variable is mutable, we can reconstruct the entire array...
    // This is lazy...?
    new_list = vec![
        list_item_1, 
        list_item_2, 
        &reversed_item, 
        list_item_4, 
        list_item_5, 
        list_item_6
        ];       


    // Print out the new list:
    println!("\n 14.2 The list now contains the reversed 3rd item : {:?}", new_list);

    // Now put the original item back.  But rather than reconstruct the entire array, we just change item 2.  We have to borrow the data &single_string_name
    // Note I tried doing the reverse in line referring to item 2, but it fails. So you have to pull it out to another variable.
    new_list[2] = &list_item_3;

    // The list contains:
    println!("\n 14.3 And now we change the list (array) back :     {:?}", new_list);



    // ################################################################################################
    // 
    // 15. Show some data types
    // 
    // v0.01 PME 2019/07/19 22:51 - Creatioon first in rust


    println!("\n -------------------------------------");
    println!("\n 15. Show some data types... \n");

    // This is done in a function.  See functions after this function.
    show_some_data_types();



    // ################################################################################################
    // 
    // 16. Call a function with some data
    // 
    // v0.01 PME 2019/07/19 22:51 - Creatioon first in rust

    // Extras - show data types.

    println!("\n -------------------------------------");
    println!("\n 16. Call a function with some data. \n");

    let more_function_data = 42;
    call_a_function_with_data(10, more_function_data);


    // ################################################################################################
    // 
    // 17. Convert some data to  and from HEX.  And use Assert to check both match.
    // 
    // v0.01 PME 2019/07/22 11:16 - Creation first in rust
    //
    // Source: https://docs.rs/hex/0.3.1/hex/fn.encode.html

    println!("\n -------------------------------------");
    println!("\n 17. Convert some data to HEX.  And use Assert to check both match. \n");

    // Setup a variable to hold the data we want to convert
    let data_to_convert_to_hex = "Hello world!";
    println!(" 17.1 Data to convert to HEX: {}", data_to_convert_to_hex);

    // Convert our data to HEX format
    extern crate hex;

    let data_converted_to_hex = hex::encode(data_to_convert_to_hex);

    // Check the process:
    // First time we use Assert here.  Assert checks the two bits of data match.
    assert_eq!(hex::encode("Hello world!"), "48656c6c6f20776f726c6421");

        // Print the hex data
    println!("      Data to converted to HEX: {}", data_converted_to_hex);

    // Convert the hex back in a print statement
    println!(" 17.2 Convert Hex back to data: {:?}", hex::decode(&data_converted_to_hex));

    // Now convert it back again using the print variable method.
    let data_converted_back_to_data = hex::decode(data_converted_to_hex);
    println!("      Convert Hex back to data: {:?}", data_converted_back_to_data);

  

    // ################################################################################################
    // 
    // 18. Convert a capital letter to lowercase.
    // 
    // v0.01 PME 2019/07/23 17:11 - Creation first in rust
    //
    // Source: https://doc.rust-lang.org/std/primitive.char.html

    println!("\n -------------------------------------");
    println!("\n 18. Convert a capital letter to lowercase. \n");

    let upper_text = "A";
    println!("      Text to convert  : {}", upper_text);

    // Convert uppercase to lowercase in a print statement.
    println!("      Converted to     : {}", upper_text.to_lowercase());

    // Now lets dom the same conversion but store the outcome.
    let convert_again = upper_text.to_lowercase();

    // An outcome that we then print with a further case inversion...
    println!("      Converted back   : {}", convert_again.to_uppercase());

    let upper_a = 'A';
    let lower_a = 'a';
    let lower_z = 'z';

    assert!(upper_a.eq_ignore_ascii_case(&lower_a));
    assert!(upper_a.eq_ignore_ascii_case(&upper_a));
    assert!(!upper_a.eq_ignore_ascii_case(&lower_z));




    // ################################################################################################
    // ################################################################################################
    // End function : main
    println!("\n -------------------------------------");
    println!("\n\n END. \n\n\n\n");

}

// metasploit







// ############################################################################################################################################
// 
// Functions


// --------------------------------------------------

// strip_characters
// Reference: https://www.rosettacode.org/wiki/Strip_a_set_of_characters_from_a_string#Rust
// 
// Strips characters from a string.
//
//      Example: let cleaned_list_string = strip_characters(&list_string, "[]\u{22} ");
//      or       println!("Cleaned List {}", strip_characters(&list_string, "[]\u{22} "));
//      or       println!("{}", strip_characters("She was a soul stripper. She took my heart!", "aei"));

fn strip_characters(original : &str, to_strip : &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
           result.push(c);
       }
    }
    result
}

// --------------------------------------------------

// find_text_in_an_array
// 
// Find content in an array of strings.
//
//      Example: find_text_in_an_array(look_at_this.to_vec(), &search_for_this);

fn find_text_in_an_array(search_from_this : Vec<&str> , search_for_this_data : &str) {
    if search_from_this.contains(&search_for_this_data) {                                           // This is a boolean test. True or False result.
        println!("     Test result {}", search_from_this.contains(&search_for_this_data));          // Print the boolean outcome of the test.
        println!("     Great news! We found {} in the list. \n", search_for_this_data);             // If True, this happens.

    } else {
        println!("     Test result {}", search_from_this.contains(&search_for_this_data));          // Print the boolean outcome of the test.
        println!("     We didn't find {}. Sorry! \n",  search_for_this_data);                       // If False, this happens.
    }
}

// --------------------------------------------------

// find_text_in_an_array
// 
// Find content in an array of strings.
//
//      Example: find_text_in_an_array(look_at_this.to_vec(), &search_for_this);


fn show_some_data_types() {
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!(" ints {:?}", ints);
    println!(" floats {:?}", floats);
    println!(" strings {:?}", strings);
    println!(" ints_ints {:?}", ints_ints);   
}

// --------------------------------------------------

// call_a_function_with_data
// Used in example 16.

// Show how you can undertake functions in print statements.
//
//      call_a_function_with_data(variable_name_containing_data, 42);

// we define the function and specify the inputs named x and y as i32 - or integer, 32 bit number.
fn call_a_function_with_data(x: i32, y: i32) {

    println!(" First number recieved is {}, and the second number is {}", x, y);
    println!(" Sum of both is: {}", x + y);
}

// --------------------------------------------------

