// Бележка: името на проекта трябва да се казва "solutionn". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_basic() {
    let input: Vec<char> = "TGAC".chars().collect();
    let counter = counts(&input);

    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 1);
    assert_eq!(counter.t, 1);

//    assert_eq!(dna_complement(&input),         vec!['C', 'G']);
//    assert_eq!(reverse_rna_complement(&input), vec!['G', 'C']);

    assert_eq!(dna_complement(&input),          vec!['A', 'C', 'T', 'G']);
    assert_eq!(reverse_rna_complement(&input),  vec!['G', 'U', 'C', 'A'])
}
