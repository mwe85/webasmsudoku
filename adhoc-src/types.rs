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


    pub struct Cell{
        
    }

    pub struct Board{
        cells: [Cell; 81]
    }
}