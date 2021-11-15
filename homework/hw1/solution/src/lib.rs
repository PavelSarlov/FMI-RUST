#[derive(Debug)]
pub struct NucleotideCounter {
    pub a: usize,
    pub c: usize,
    pub g: usize,
    pub t: usize,
}

pub fn counts(dna: &[char]) -> NucleotideCounter {
    let mut result = NucleotideCounter {
        a: 0,
        c: 0,
        g: 0,
        t: 0,
    };

    for nucl in dna {
        match nucl {
            'A' => result.a += 1,
            'C' => result.c += 1,
            'G' => result.g += 1,
            'T' => result.t += 1,
            _ => panic!("Invalid nucleotide."),
        }
    }

    result
}

pub fn dna_complement(dna: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::with_capacity(dna.len());

    for n in dna {
        let nucl: char = match n {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => panic!("Invalid nucleotide."),
        };
        result.push(nucl);
    }

    result
}

pub fn reverse_rna_complement(dna: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::with_capacity(dna.len());
    
    for n in dna.iter().rev() {
        let nucl: char = match n {
            'A' => 'U',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => panic!("Invalid nucleotide."),
        };
        result.push(nucl);
    }

    result
}
