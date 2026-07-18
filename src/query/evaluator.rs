pub create::modeals::posting::Posting;

pub struct PostingList{
    pub postings:Vec<Posting>
}

impl PostingList{

    pub fn intesection() -> PostingList{
    }

    pub fn union() -> PostingList{
    }

    pub fn difference() -> PostingList{
    }

}

pub fn evaluate(node:AstNode,idx: InvertedIndex ) ->Vec<usize> {
    // Ast =>  Not(And(Rust,Or(File,Name)))
    match node{
    }
}
