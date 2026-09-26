//! Many-minds fork cost (0.29): how much one more [`Session::fork`] costs
//! when N forks already live off one grounded world, N in {1, 8, 64, 512}.
//!
//! Reported per N:
//!
//! - `world_bytes`: the shared grounded world (paid once, whatever N is);
//! - `mind_bytes`: one fork's private state (the per-fork toll), and the
//!   N-fork total;
//! - fork latency: the criterion group `session_fork/fork_n`, which times
//!   creating N forks from one parent (throughput = N elements, so the
//!   per-fork cost reads directly off the report).
//!
//! The byte figures are deterministic (the session's own flat-bytes model),
//! so they are printed once per N rather than timed. The world is a
//! generated 64-room corridor (inline, no external corpus) so the grounded
//! world is large enough that sharing it is the thing being measured.
//! Run: `cargo bench -p ferroplan --bench session_fork` (`-- --quick` for a
//! fast pass, `-- --test` to execute each benchmark once).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ferroplan::{Options, Session};
use std::hint::black_box;

const ROOMS: usize = 64;
const FORK_COUNTS: [usize; 4] = [1, 8, 64, 512];

const DOMAIN: &str = "(define (domain rooms)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b))
    :effect (and (at ?b) (not (at ?a)))))";

fn corridor_problem(rooms: usize) -> String {
    let objects: Vec<String> = (0..rooms).map(|i| format!("r{i}")).collect();
    let links: Vec<String> = (0..rooms - 1)
        .map(|i| format!("(link r{i} r{})", i + 1))
        .collect();
    format!(
        "(define (problem corridor-{rooms})
  (:domain rooms)
  (:objects {} - room)
  (:init (at r0) {})
  (:goal (at r{})))",
        objects.join(" "),
        links.join(" "),
        rooms - 1
    )
}

fn parent() -> Session {
    Session::new(
        DOMAIN,
        &corridor_problem(ROOMS),
        &Options {
            threads: 1,
            ..Default::default()
        },
    )
    .expect("generated corridor grounds")
}

fn session_fork(c: &mut Criterion) {
    let parent = parent();
    for n in FORK_COUNTS {
        let forks: Vec<Session> = (0..n).map(|_| parent.fork()).collect();
        let world = parent.world_bytes();
        let mind = forks[0].mind_bytes();
        let total_mind: usize = forks.iter().map(Session::mind_bytes).sum();
        println!(
            "session_fork n={n} world_bytes={world} mind_bytes={mind} \
             total_mind_bytes={total_mind} world_shared_once=true"
        );
    }

    let mut group = c.benchmark_group("session_fork");
    for n in FORK_COUNTS {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("fork_n", n), &n, |b, &n| {
            b.iter(|| {
                let forks: Vec<Session> = (0..n).map(|_| parent.fork()).collect();
                black_box(forks)
            })
        });
    }
    group.finish();
}

criterion_group!(benches, session_fork);
criterion_main!(benches);
