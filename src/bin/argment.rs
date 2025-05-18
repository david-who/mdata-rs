use argh::FromArgs;

/// Command to manage a classroom.
#[derive(Debug, PartialEq, FromArgs)]
struct ClassroomCmd {
    #[argh(subcommand)]
    subcommands: Subcommands,
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Subcommands {
    List(ListCmd),
    Add(AddCmd),
}

/// list all the classes.
#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand, name = "list")]
struct ListCmd {
    /// list classes for only this teacher.
    #[argh(option)]
    teacher_name: Option<String>,
}

/// add students to a class.
#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand, name = "add")]
struct AddCmd {
    /// the name of the class's teacher.
    #[argh(option)]
    teacher_name: String,

    /// the name of the class.
    #[argh(positional)]
    class_name: String,
}


////////////////////////////////////////////////////////////////

#[derive(Debug, FromArgs)]
/// mdata test parameters
struct MDP {
    /// remote target
    #[argh(option)]
    target: Option<String>,
    /// cmd
    #[argh(option, default = "3")]
    cmd: u32,
    /// time in ms between two ticks.
    #[argh(option, default = "1702")]
    port: u64,
    /// verbose output
    #[argh(option, short = 'v', default = "false")]
    verbose: bool,
}

////////////////////////////////////////////////////////////////


fn main()
{
    let param: MDP = argh::from_env();
    
    let args = ClassroomCmd::from_args(
        &["classroom"],
        &["list", "--teacher-name", "Smith"],
    ).unwrap();
    assert_eq!(
       args,
        ClassroomCmd {
            subcommands: Subcommands::List(ListCmd {
                teacher_name: Some("Smith".to_string()),
            })
        },
    );
    
    // Help returns an error, but internally returns an `Ok` status.
    let early_exit = ClassroomCmd::from_args(
        &["classroom"],
        &["help"],
    ).unwrap_err();
    assert_eq!(
        early_exit,
        argh::EarlyExit {
           output: r#"Usage: classroom <command> [<args>]
    
    Command to manage a classroom.
    
    Options:
      --help            display usage information
    
    Commands:
      list              list all the classes.
      add               add students to a class.
    "#.to_string(),
           status: Ok(()),
        },
    );
    
    // Help works with subcommands.
    let early_exit = ClassroomCmd::from_args(
        &["classroom"],
        &["list", "help"],
    ).unwrap_err();
    assert_eq!(
        early_exit,
        argh::EarlyExit {
           output: r#"Usage: classroom list [--teacher-name <teacher-name>]
    
    list all the classes.
    
    Options:
      --teacher-name    list classes for only this teacher.
      --help            display usage information
    "#.to_string(),
           status: Ok(()),
        },
    );
    
    // Incorrect arguments will error out.
    let err = ClassroomCmd::from_args(
        &["classroom"],
        &["lisp"],
    ).unwrap_err();
    assert_eq!(
       err,
       argh::EarlyExit {
           output: "Unrecognized argument: lisp\n".to_string(),
           status: Err(()),
        },
    );
}