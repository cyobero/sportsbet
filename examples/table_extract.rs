use table_extract;

fn main() {
    let html = "https://www.basketball-reference.com/teams/BOS/2022_games.html#games";
    let table = table_extract::Table::find_first(html).unwrap();
}
