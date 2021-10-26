pub struct NucleotideCounter {
    pub a: usize,
    pub c: usize,
    pub g: usize,
    pub t: usize,
}

pub fn counts(dna: &[char]) -> NucleotideCounter {
    let mut counter = NucleotideCounter{
        a:  0,
        c:  0,
        g:  0,
        t:  0,
    };

    for i in dna{
        if *i == 'A'{
            counter.a = counter.a + 1;
        }
        else if *i == 'C'{
            counter.c = counter.c + 1;
        }
        else if *i == 'G'{
            counter.g = counter.g + 1;
        }
        else if *i == 'T'{
            counter.t = counter.t + 1;
        }
        else{
            panic!("There is no such  nucleotide bases");
        }
    }
    counter
}

pub fn dna_complement(dna: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for i in dna {
        if *i == 'A'{
            result.push('T');
        }
        else if *i == 'C'{
            result.push('G');
        }
        else if *i == 'G'{
            result.push('C');
        }
        else if *i == 'T'{
            result.push('A')
        }
        else{
            panic!("There is no such  nucleotide bases");
        } 
    }

    result
}

pub fn reverse_rna_complement(dna: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for i in dna {
        if *i == 'A'{
            result.push('U');
        }
        else if *i == 'C'{
            result.push('G');
        }
        else if *i == 'G'{
            result.push('C');
        }
        else if *i == 'T'{
            result.push('A')
        }
        else{
            panic!("There is no such  nucleotide bases");
        } 
    }
    result.reverse();

    result
}