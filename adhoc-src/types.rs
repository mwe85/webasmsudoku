pub mod BoardTypes{
    pub type HintSet = u32;

    //HintSet, because it is a primitive type, cannot have
    //an implementation. It needs an extension trait.

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