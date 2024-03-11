struct StatefulList {
    state: ListState,
    items: Vec<News>,
    last_selected: Option<usize>,
}
