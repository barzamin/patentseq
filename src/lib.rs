use biotypes::{AminoAcid, AsTriplet};
use std::io::{self, Write};

pub mod metadata;

pub trait PatentSerialize {
    fn write_patent_form<W>(&self, w: &mut W) -> io::Result<()>
    where
        W: Write;
}

fn prot_write_line<'a, I, W>(iter: &mut I, w: &mut W) -> io::Result<()>
where
    I: Iterator<Item = &'a AminoAcid>,
    W: Write,
{
    let mut iter = iter.take(16).peekable();
    while let Some(aa) = iter.next() {
        write!(w, "{}", aa.as_triplet())?;
        if iter.peek().is_some() {
            write!(w, " ")?;
        }
    }

    Ok(())
}

fn prot_write_ruler<W>(w: &mut W, base_aa_idx: usize, prot_len: usize) -> io::Result<()>
where
    W: Write,
{
    let mut line_aa_idx = 0;
    while line_aa_idx < 16 && (base_aa_idx + line_aa_idx) <= prot_len {
        let aa_idx = base_aa_idx + line_aa_idx;
        if aa_idx % 5 == 0 || aa_idx == 1 {
            write!(w, "{:>3}", aa_idx)?;
        } else {
            write!(w, "   ")?;
        }

        if line_aa_idx < 15 {
            write!(w, " ")?;
        }

        line_aa_idx += 1;
    }
    Ok(())
}

impl PatentSerialize for Vec<AminoAcid> {
    fn write_patent_form<W>(&self, w: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        let mut aas = self.iter().peekable();

        let mut line_idx = 1;
        while aas.peek().is_some() {
            prot_write_line(&mut aas, w)?;
            writeln!(w)?;
            prot_write_ruler(w, line_idx, self.len())?;
            line_idx += 16;
            writeln!(w)?;
            writeln!(w)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn smokeout() {
        use AminoAcid::*;
        let x = vec![
            M, F, V, F, L, V, L, L, P, L, V, S, S, Q, C, V, N, L, T, T, R, T, Q, L, P, P, A, Y, T,
            N, S, F, T, R, G, V, Y, Y, P, D, K, V, F, R, S, S, V, L, H, S, T, Q, D, L, F, L, P, F,
            F, S, N, V, T, W, F, H, A, I, H, V,
        ];

        let mut w = std::io::stdout();
        x.write_patent_form(&mut w)
            .expect("writing patent form failed");
        w.flush().unwrap();

        panic!();
    }
}
