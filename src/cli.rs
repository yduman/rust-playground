use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("\n ==> You need to provide at least one argument. Run: 'cargo run hello'\n")
    }

    let command = args[1].clone();

    if command == "hello" {
        print!(
            "--------------------------------------------------------------------------------\n\
             user's full name (user):              {}\n\
             username (username):                  {}\n\
             --------------------------------------------------------------------------------\n\
             host's fancy name (host):             {}\n\
             hostname (hostname):                  {}\n\
             --------------------------------------------------------------------------------\n\
             platform (platform):                  {}\n\
             operating system (os):                {}\n\
             desktop environment (env):            {}\n\
             --------------------------------------------------------------------------------\n\
             ",
            whoami::user(),
            whoami::username(),
            whoami::host(),
            whoami::hostname(),
            whoami::platform(),
            whoami::os(),
            whoami::env(),
        );
    } else {
        println!("Please use the 'hello' command");
    }
}