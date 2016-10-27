/// This should be the top of the grammar hierarchy. Feel free to rename it,
/// extend to model your domain and make your own.
#[derive(Debug,Clone,PartialEq)]
pub enum Start {
    // TODO model your domain
    Todo,
}

impl_astnode!(Start, 1,
              leaf Todo());

// Device a means to evaluate `Start`
