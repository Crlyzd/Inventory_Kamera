//! Fuzzy dictionary matching engine using Jaro-Winkler string similarity.
#![allow(dead_code)]

use strsim::jaro_winkler;

/// A matched dictionary candidate with similarity score.
#[derive(Debug, Clone)]
pub struct MatchCandidate<'a> {
    pub matched_name: &'a str,
    pub similarity: f64,
}

/// Finds the closest matching dictionary entry above the minimum similarity threshold.
pub fn find_best_match<'a>(query: &str, candidates: &'a [&'a str], min_similarity: f64) -> Option<MatchCandidate<'a>> {
    let clean_query = query.trim().to_lowercase();
    if clean_query.is_empty() {
        return None;
    }

    let mut best: Option<MatchCandidate<'a>> = None;

    for candidate in candidates {
        let clean_candidate = candidate.trim().to_lowercase();
        let sim = jaro_winkler(&clean_query, &clean_candidate);

        if sim >= min_similarity {
            if let Some(ref current_best) = best {
                if sim > current_best.similarity {
                    best = Some(MatchCandidate {
                        matched_name: candidate,
                        similarity: sim,
                    });
                }
            } else {
                best = Some(MatchCandidate {
                    matched_name: candidate,
                    similarity: sim,
                });
            }
        }
    }

    best
}
