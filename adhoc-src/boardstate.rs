/***
  adhoc-src is temporary location for these files until i can use rust build tools to create/structure a project.
  
  implementation goals
***/
pub mod proto1{
  pub mod BoardComponents{
    struct RCSConstraint{
      rows: [usize; 8],
      cols: [usize; 8],
      sqrs: [usize; 8]
    }; 
  }
}