
mod quick_sort;
mod insertion_sort;


type VariableId = usize;

type Float = f64;


pub struct ScoredParents {
    pub parents: Vec<VariableId>,
    pub score: Float,
}


pub trait Toplist {

    fn empty_score(&self) -> Float;

    fn add(&mut self, family: &ScoredParents) -> Option<Vec<ScoredParents>> {
        if family.parents.is_empty() || family.score <= self.empty_score() {
            None
        } else {
            self.add_family(family)
        }
    }

    fn add_family(&mut self, family: &ScoredParents) -> Option<Vec<ScoredParents>>;

}


struct ToplistWithoutFilter {
    variable_id: VariableId,
    empty_score: Float,
}


struct ToplistWithFilter {
    variable_id: VariableId,
    empty_score: Float,
}
