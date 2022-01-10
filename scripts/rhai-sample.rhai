fn handle_event(data) {
    if type_of(data) == "i64" {
       print("number! " + data);
    } else {
       print("text! " + data);
    }
    print_fancy("got an event! " + name);
}