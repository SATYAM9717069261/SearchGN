use crate::models::posting::Posting;
use crate::query::operator::Operator;
use crate::query::ast::AstNode;
use crate::indexer::inverted_index::InvertedIndex;

#[derive(Debug)]
pub struct PostingList{
    pub postings:Vec<Posting>
}

fn build_universe(index: &InvertedIndex) -> PostingList {
    let mut result = PostingList::new();
    for doc_id in 0..index.get_all_document().len() {
        result.postings.push(
            Posting::new(doc_id, 0, Vec::new(), Vec::new())
        );
    }
    result
}

impl PostingList{

    pub fn intesection(&self,list2:&PostingList) -> PostingList{
        let mut result = PostingList{postings: Vec::new()};
        let mut idx_l1 = 0; // self.postings
        let mut idx_l2 = 0;
        while idx_l1 < self.postings.len() && idx_l2 < list2.postings.len(){
            let l1_doc_id = self.postings[idx_l1].get_document_id();
            let l2_doc_id = list2.postings[idx_l2].get_document_id();
            if l1_doc_id ==  l2_doc_id{
                result.postings.push(self.postings[idx_l1].clone());
                idx_l1+=1; idx_l2+=1;
            }else if l1_doc_id <  l2_doc_id{
                idx_l1+=1;
            }else{
                idx_l2+=1;
            }
        }
        return result;
    }

    pub fn union(&self,list2:&PostingList) -> PostingList{
        let mut result = PostingList{postings: Vec::new()};
        let mut idx_l1 = 0; // self.postings
        let mut idx_l2 = 0;
        while idx_l1 < self.postings.len() && idx_l2 < list2.postings.len(){
            let l1_doc_id = self.postings[idx_l1].get_document_id();
            let l2_doc_id = list2.postings[idx_l2].get_document_id();
            if l1_doc_id ==  l2_doc_id{
                result.postings.push(self.postings[idx_l1].clone());
                idx_l1+=1;
                idx_l2+=1;
            }else if l1_doc_id <  l2_doc_id{
                result.postings.push(self.postings[idx_l1].clone());
                idx_l1+=1;
            }else{
                result.postings.push(list2.postings[idx_l2].clone());
                idx_l2+=1;
            }
        }

        while idx_l1 < self.postings.len(){
            result.postings.push(self.postings[idx_l1].clone());
            idx_l1+=1;
        }

        while idx_l2 < list2.postings.len(){
            result.postings.push(list2.postings[idx_l2].clone());
            idx_l2+=1;
        }

        return result;
    }

    pub fn not( &self, list2: &PostingList) -> PostingList{
        let mut result = PostingList{postings: Vec::new()};
        let mut idx_l1 = 0; // self.postings
        let mut idx_l2 = 0;
        while idx_l1 < self.postings.len() && idx_l2 < list2.postings.len(){
            let l1_doc_id = self.postings[idx_l1].get_document_id();
            let l2_doc_id = list2.postings[idx_l2].get_document_id();
            if l1_doc_id == l2_doc_id{
                idx_l1+=1;
                idx_l2+=1;
            }else if l1_doc_id <  l2_doc_id{
                result.postings.push(self.postings[idx_l1].clone());
                idx_l1+=1;
            }else{
                idx_l2+=1;
            }
        }

        while idx_l1 < self.postings.len(){
            result.postings.push(self.postings[idx_l1].clone());
            idx_l1+=1;
        }

        result
    }

    pub fn new() -> Self{
        Self{
            postings: Vec::new()
        }
    }
}

pub fn evaluate(node:&AstNode,idx: &InvertedIndex ) ->PostingList {
    // node => Binary { operator: And, left: Word("word"), right: Word("too") }

    match node{
        AstNode::Word(wrd) => {
            match idx.search_word(wrd){
                Ok(postings) => PostingList{
                    postings:  postings.clone()
                },
                Err(er) => PostingList::new()
            }
        }
        AstNode::Binary{ operator, left, right} => {
            let left = evaluate(left,idx);
            let right = evaluate(right,idx);

            match operator{
                Operator::And => left.intesection(&right),
                Operator::Or => left.union(&right),
                _ => unreachable!()
            }
        },
        AstNode::Unary{ operator, child } =>{
            let child = evaluate(child,idx);
            match operator{
                Operator::Not => {
                    let universe = build_universe(idx);
                    universe.not(&child)
                },
                _ => unreachable!()
            }
        }
    }
}
