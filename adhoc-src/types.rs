pub mod BoardTypes{

    pub struct RCSConstraint{
        rows: [usize; 9],
        cols: [usize; 9],
        sqs: [usize; 9]
    }

    impl RCSConstraint{
        pub fn new(self){
            //initialize all these arrays
        }
    }

    pub type HintSet = u32;

    //HintSet, because it is a primitive type, cannot have
    //an implementation. It needs an extension trait.

    /*https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html*/
    trait HintSetEx{
       fn new(self) -> HintSet;
        fn isValue(self) -> bool;
    }

    impl HintSetEx for HintSet{
        fn new(self) -> HintSet{return 0}

        fn isValue(self) -> bool{return false;}
    }

    /**
    *CellAddress e {0...80}
    */
    pub type CellAddress = usize;

    pub struct Cell{
        value: HintSet,

        row_prev: CellAddress,
        row_next: CellAddress,
        
        col_prev: CellAddress,
        col_next: CellAddress,

        sq_next: CellAddress,
        sq_prev: CellAddress,
    }

    pub struct Board{
        cells: [Cell; 81]
    }
}