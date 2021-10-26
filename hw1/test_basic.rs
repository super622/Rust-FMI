use solution::*;

#[test]
fn test_basic() {
    let input: Vec<char> = "GC".chars().collect();
    let counter = solution::counts(&input);
    
    //let test2: Vec<char> = "ATGCa".chars().collect();
    //let test3: Vec<char> = "ATCGAGTCAG".chars().collect();

    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 0);
    assert_eq!(counter.t, 0);

    /*let counter = solution::counts(&test2);
    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 1);
    assert_eq!(counter.t, 1);


    let counter = solution::counts(&test3);
    assert_eq!(counter.g, 3);
    assert_eq!(counter.c, 2);
    assert_eq!(counter.a, 3);
    assert_eq!(counter.t, 2);
    
    assert_eq!(solution::dna_complement(&test3), vec!['T','A','G','C','T','C','A','G','T','C']);
    assert_eq!(solution::reverse_rna_complement(&test3),vec!['C','U','G','A','C','U','C','G','A','U']);*/
    assert_eq!(solution::dna_complement(&input),         vec!['C', 'G']);
    assert_eq!(solution::reverse_rna_complement(&input), vec!['G', 'C']);
}