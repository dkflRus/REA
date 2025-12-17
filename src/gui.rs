
enum GUIInputType{
    // EventTable, //TODO: Allow multiple EventTables as input for more complex piping
    bool,
    u64,
    f64,
    String,
    ComboBox(Vec<String>)
}
// pub type GUIInputList=Vec<(/*Name:*/&'static String,InputType)>;