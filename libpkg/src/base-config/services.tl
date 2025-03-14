{
    userd = {
        name = "User Daemon"
        exec = "userd"
    }
    login = {
        name = "Login Prompt"
        exec = "login"
        dependencies = [ "userd" ]
        io = [ "in" "out" "err" ]
    }
}
