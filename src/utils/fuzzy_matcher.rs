use strsim::levenshtein;
use std::cmp;

pub fn suggest_similar<'a,I>(input: &str, candidates: I) -> Vec<String> 
where
    I: Iterator<Item = &'a String>,
{
    let mut suggestions: Vec<(String, usize)> = candidates.map(|candidate| (candidate.clone(), levenshtein(input, &candidate))).collect();
    suggestions.sort_by(|a, b| a.1.cmp(&b.1));
    
    if suggestions.is_empty() {
        return Vec::new();
    }

    let (_ ,min_distance) = suggestions.first().unwrap();
    let threshold = cmp::max((input.len() as f64 * 0.2).ceil() as usize, 2);

    if *min_distance > threshold {
        return Vec::new();
    }

    suggestions.iter()
        .filter(|&&(_, distance)| distance == *min_distance)
        .map(|(candidate, _)| candidate.clone())
        .collect()
}




#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_suggest_similar_single_match() {
        let input = "appl";
        let binding = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        let candidates = binding.iter();
        let result:Vec<String> = vec!["apple"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(result, suggest_similar(input, candidates));
    }

    #[test]
    fn test_suggest_similar_multiple_matches() {
        let input = "appl";
        let binding = vec!["apply".to_string(), "apple".to_string(), "banana".to_string()];
        let candidates = binding.iter();
        let result:Vec<String> = vec!["apply", "apple"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(result, suggest_similar(input, candidates));
    }

    #[test]
    fn test_suggest_similar_no_matches() {
        let input = "xyz";
        let binding = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        let candidates = binding.iter();
        let result:Vec<String> = vec![];
        assert_eq!(result, suggest_similar(input, candidates));
    }


    #[test]
    fn test_suggest_id()    {
        let input = "id";
        let binding = vec!["id1".to_string()];
        let candidates = binding.iter();
        let result:Vec<String> = vec!["id1".to_string()];
        assert_eq!(result, suggest_similar(input, candidates));
    }
}
