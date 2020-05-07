pub trait Asletter {
    fn as_letter(&self) -> &str;
}

pub trait AsTriplet {
    fn as_triplet(&self) -> &str;
}

pub trait AsFullName {
    fn as_fullname(&self) -> &str;
}

pub enum Nucleobase {
    A,
    C,
    G,
    T,
    U,
}

pub enum AminoAcid {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    S,
    T,
    V,
    W,
    X,
    Y,
    Z,
}

impl Asletter for AminoAcid {
    fn as_letter(&self) -> &'static str {
        use AminoAcid::*;
        match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            H => "H",
            I => "I",
            K => "K",
            L => "L",
            M => "M",
            N => "N",
            P => "P",
            Q => "Q",
            R => "R",
            S => "S",
            T => "T",
            V => "V",
            W => "W",
            X => "X",
            Y => "Y",
            Z => "Z",
        }
    }
}

impl AsTriplet for AminoAcid {
    fn as_triplet(&self) -> &'static str {
        use AminoAcid::*;
        match self {
            A => "Ala",
            B => "Asx",
            C => "Cys",
            D => "Asp",
            E => "Glu",
            F => "Phe",
            G => "Gly",
            H => "His",
            I => "Ile",
            K => "Lys",
            L => "Leu",
            M => "Met",
            N => "Asn",
            P => "Pro",
            Q => "Gln",
            R => "Arg",
            S => "Ser",
            T => "Thr",
            V => "Val",
            W => "Trp",
            X => "Xaa",
            Y => "Tyr",
            Z => "Glx",
        }
    }
}

impl AsFullName for AminoAcid {
    fn as_fullname(&self) -> &'static str {
        use AminoAcid::*;
        match self {
            A => "Alanine",
            B => "Aspartic acid or Asparagine",
            C => "Cysteine",
            D => "Aspartic Acid",
            E => "Glutamic Acid",
            F => "Phenylalanine",
            G => "Glycine",
            H => "Histidine",
            I => "Isoleucine",
            K => "Lysine",
            L => "Leucine",
            M => "Methionine",
            N => "Asparagine",
            P => "Proline",
            Q => "Glutamine",
            R => "Arginine",
            S => "Serine",
            T => "Threonine",
            V => "Valine",
            W => "Tryptophan",
            X => "Any amino acid",
            Y => "Tyrosine",
            Z => "Glutamine or Glutamic acid",
        }
    }
}

