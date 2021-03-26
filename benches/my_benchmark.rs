use criterion::{black_box, criterion_group, criterion_main, Criterion};


use games::abstractions::play;
use games::abstractions::Environment;

use games::othello::Board;
use games::othello::AgentId;
use games::tree_search::greedy_reward;
use games::agents::alphabeta_agent::AlphabetaAgent;

fn criterion_benchmark(c: &mut Criterion) {
    let mut board = Board::initial_state();

    let mut player_w = AlphabetaAgent::new(AgentId::W, &greedy_reward, 3);
    let mut player_b = AlphabetaAgent::new(AgentId::B, &greedy_reward, 3);

    c.bench_function("play", |b| b.iter(|| play(black_box(&mut board), black_box(&mut player_w), black_box(&mut player_b))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);