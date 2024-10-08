use crate::de::{ArticleId, AuthorList, Date, GrantList, JournalIssue, KeywordList, PublicationType, PubmedArticle, PubmedArticleSet, ReferenceList};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case"))]
struct Journal {
    /// from MedlineJournalInfo
    id: String,
    country: String,
    issn: String,
    /// from Journal
    title: String,
    // iso_abbreviation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    journal_issue: Option<JournalIssue>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case"))]
struct Reference {
    cite: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    refs: Vec<ArticleId>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case"))]
struct Article {
    id: u64,
    title: String,
    pub_model: String,
    date_created: Date,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_completed: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    keywords: Option<KeywordList>,
    journal: Journal,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    authors: Option<AuthorList>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    publication_types: Vec<PublicationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    grants: Option<GrantList>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    references: Vec<String>,
    // references: Vec<Reference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    article_ids: Vec<ArticleId>,
}

impl From<&PubmedArticle> for Article {
    fn from(value: &PubmedArticle) -> Self {
        let pubmed_data = &value.pubmed_data;
        let medline_citation = &value.medline_citation;
        let medline_journal_info = medline_citation.medline_journal_info.clone();
        let article_journal = medline_citation.article.journal.clone();
        let rf = &pubmed_data.reference_list;
        Article {
            id: medline_citation.id.id,
            title: medline_citation.article.article_title.clone(),
            pub_model: medline_citation.article.pub_model.clone(),
            keywords: medline_citation.keyword_list.clone(),
            journal: Journal {
                id: medline_journal_info.id,
                country: medline_journal_info.country,
                issn: medline_journal_info.issn,
                title: article_journal.title,
                // iso_abbreviation: article_journal.iso_abbreviation,
                journal_issue: article_journal.journal_issue,
            },
            authors: medline_citation.article.author_list.clone(),
            date_created: medline_citation.date_revised.clone(),
            date_completed: medline_citation.date_completed.clone(),
            publication_types: medline_citation.article.publication_type_list.publication_type.clone(),
            grants: medline_citation.article.grant_list.clone(),
            references: process_references(
                &(if rf.is_empty() {
                    None
                } else {
                    Some(rf[0].clone())
                }
                )),
            article_ids: pubmed_data.article_id_list.article_id.clone(),
        }
    }
}

fn process_references(input: &Option<ReferenceList>) -> Vec<String> {
    // pubmed_data.reference_list.clone()
    //     .map_or(vec![],
    //             |e| e.reference.iter().map(
    //                 |x| Reference {
    //                     cite: x.citation.clone(),
    //                     refs: x.article_id_list.clone().map_or(vec![], |e| e.article_id),
    //                 }
    //             ).collect()),
    match input {
        Some(input) => {
            let mut res: Vec<String> = vec![];
            for r in &input.reference {
                match &r.article_id_list {
                    None => continue,
                    Some(article_ids) => {
                        let mut p: Vec<String> =
                            article_ids.article_id.iter()
                                .filter(|e| e.ty == "pubmed")
                                .map(|e| e.id.clone()).collect();
                        res.append(&mut p);
                    }
                }
            }
            res
        }
        None => vec![],
    }
}

fn run_de_ser(content: &str) -> Vec<String> {
    let xd = &mut quick_xml::de::Deserializer::from_str(content);
    let res: Result<PubmedArticleSet, _> = serde_path_to_error::deserialize(xd);
    res.unwrap().pubmed_article.iter()
        .map(|e| Article::from(e))
        .map(|e| serde_json::ser::to_string(&e).unwrap())
        .collect()
}

mod tests {
    use crate::ser::run_de_ser;
    use std::fs;
    use std::io::{LineWriter, Write};

    #[test]
    fn de_ser_test() {
        // let filepath = "test";
        let filepath = r"pubmed24n1212";
        let content = fs::read_to_string(format!("{}.xml", filepath)).unwrap();
        let deser = run_de_ser(&content);
        println!("{}: {}", filepath, deser.len());
        // deser.iter().take(3).for_each(|e| println!("{}", e));
        let file = fs::File::create(format!("{}.ndjson", filepath)).unwrap();
        let mut file = LineWriter::new(file);
        for (i, str) in deser.iter().enumerate() {
            print!("{} ", i);
            file.write_all(str.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }
}