# ReadMe

Add your library like this :
Please paste your .rs file without any main function in the src folder. Then in src, add your file in lib.rs, save and close it.
Also in the main file in src, please add use etig::<your_file>, (see lines 15-21 in main file). After this, go to main function, call your function like this:
your_file_name::function_name(&mut graph, na);
Also make sure the calling function is declared like this in your module:
pub fn function_name(args<>) {}
{See the main file for further help, or praneel.rs or samarth.rs for running examples.}
We have given some inputs but if you want to give any others, please feel free to paste your input files out of the src folder.
To run or test your files, run the command :-
cargo build
cargo run input_file.txt
