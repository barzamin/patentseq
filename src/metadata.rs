use std::fmt;
use std::io::Write;

use biotypes::Locus;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum SeqType {
    Dna,
    Rna,
    Protein,
}

impl fmt::Display for SeqType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            SeqType::Dna => "DNA",
            SeqType::Rna => "RNA",
            SeqType::Protein => "PRT",
        })
    }
}

#[derive(Debug)]
pub enum IdentLine {
    Applicant(String),
    InventionTitle(String),

    FileReference(String),

    ApplicationNumber(String),
    FilingDate(NaiveDate),

    PriorApplicationNumber(String),
    PriorApplicationDate(NaiveDate),

    SeqIdCount(usize),
    Software(String),
    SeqIdNumber(usize),

    Length(usize),
    Type(SeqType),
    Organism(String),

    FeatureName(String),
    FeatureHeader,
    FeatureLocation(Locus),
    FeatureOtherInfo(String),

    PublicationHeader,
    PublicationAuthors(String),
    PublicationTitle(String),
    PublicationJournal(String),
    PublicationVolume(String),
    PublicationIssue(String),
    PublicationPages(String),
    PublicationDate(NaiveDate),

    DatabaseAccession(String),
    DatabaseEntryDate(NaiveDate),

    DocumentNumber(String),
    DocumentFilingDate(NaiveDate),
    DocumentPublicationDate(NaiveDate),
    RelevantResidues(Locus),

    Sequence(usize),
}

impl IdentLine {
    pub fn raw_code(&self) -> u16 {
        match self {
            IdentLine::Applicant(_) => 110,
            IdentLine::InventionTitle(_) => 120,

            IdentLine::FileReference(_) => 130,

            IdentLine::ApplicationNumber(_) => 140,
            IdentLine::FilingDate(_) => 141,

            IdentLine::PriorApplicationNumber(_) => 150,
            IdentLine::PriorApplicationDate(_) => 151,

            IdentLine::SeqIdCount(_) => 160,

            IdentLine::Software(_) => 170,

            IdentLine::SeqIdNumber(_) => 210,
            IdentLine::Length(_) => 211,
            IdentLine::Type(_) => 212,
            IdentLine::Organism(_) => 213,

            IdentLine::FeatureHeader => 220,
            IdentLine::FeatureName(_) => 221,
            IdentLine::FeatureLocation(_) => 222,
            IdentLine::FeatureOtherInfo(_) => 223,

            IdentLine::PublicationHeader => 300,
            IdentLine::PublicationAuthors(_) => 301,
            IdentLine::PublicationTitle(_) => 302,
            IdentLine::PublicationJournal(_) => 303,
            IdentLine::PublicationVolume(_) => 304,
            IdentLine::PublicationIssue(_) => 305,
            IdentLine::PublicationPages(_) => 306,
            IdentLine::PublicationDate(_) => 307,

            IdentLine::DatabaseAccession(_) => 308,
            IdentLine::DatabaseEntryDate(_) => 309,

            IdentLine::DocumentNumber(_) => 310,
            IdentLine::DocumentFilingDate(_) => 311,
            IdentLine::DocumentPublicationDate(_) => 312,

            IdentLine::RelevantResidues(_) => 313,
            IdentLine::Sequence(_) => 400,
        }
    }

    pub fn serialize<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        write!(w, "<{}>\t", self.raw_code())?;

        match self {
            IdentLine::Applicant(applicant) => write!(w, "{}", applicant),
            IdentLine::InventionTitle(title) => write!(w, "{}", title),

            IdentLine::ApplicationNumber(app_no) => write!(w, "{}", app_no),
            IdentLine::FilingDate(filing_date) => write!(w, "{}", filing_date.format("%Y-%m-%d")),

            IdentLine::SeqIdCount(count) => write!(w, "{}", count),

            IdentLine::SeqIdNumber(id) => write!(w, "{}", id),
            IdentLine::Length(l) => write!(w, "{}", l),
            IdentLine::Type(ty) => write!(w, "{}", ty),
            IdentLine::Organism(org) => write!(w, "{}", org),

            IdentLine::Sequence(id) => write!(w, "{}", id),

            _ => unimplemented!(),
        }
    }
}
