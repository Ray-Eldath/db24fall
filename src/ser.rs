use crate::de::{
    ArticleId, AuthorList, Date, GrantList, JournalIssue, Keyword, KeywordList, PublicationType,
    PubmedArticle, PubmedArticleSet, ReferenceList,
};
use crate::stats::STATS;
use serde::Serialize;
use std::sync::atomic::Ordering;

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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    keywords: Vec<Keyword>,
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
            keywords: medline_citation
                .keyword_list
                .iter()
                .flat_map(|lst| lst.keyword.clone())
                .collect(),
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
            publication_types: medline_citation
                .article
                .publication_type_list
                .publication_type
                .clone(),
            grants: medline_citation.article.grant_list.clone(),
            references: process_references(
                medline_citation.id.id,
                &(if rf.is_empty() {
                    None
                } else {
                    Some(rf[0].clone())
                }),
            ),
            article_ids: pubmed_data.article_id_list.article_id.clone(),
        }
    }
}

fn process_references(self_id: u64, input: &Option<ReferenceList>) -> Vec<String> {
    match input {
        Some(input) => {
            let mut res: Vec<String> = vec![];
            for r in &input.reference {
                match &r.article_id_list {
                    None => continue,
                    Some(article_ids) => {
                        let vec = &article_ids.article_id;
                        STATS.refs_before_filtering.fetch_add(vec.len(), Ordering::SeqCst);
                        let mut p: Vec<String> = vec.iter()
                            .filter(|e| e.ty == "pubmed" && e.id.is_some())
                            .map(|e| e.id.clone().unwrap())
                            .filter(|e| e.parse::<u64>().unwrap() <= 3024180).collect();
                        STATS.refs_after_filtering.fetch_add(p.len(), Ordering::SeqCst);
                        res.append(&mut p);
                    }
                }
            }
            res
        }
        None => vec![],
    }
}

pub(crate) fn run_de_ser(content: &str) -> Vec<String> {
    let xd = &mut quick_xml::de::Deserializer::from_str(content);
    let res: Result<PubmedArticleSet, _> = serde_path_to_error::deserialize(xd);
    res.unwrap().pubmed_article.iter()
        .map(|e| Article::from(e))
        .map(|e| serde_json::ser::to_string(&e).unwrap())
        .collect()
}
