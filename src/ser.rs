use crate::de::{ArticleId, AuthorList, Date, GrantList, Journal, PublicationType, PubmedArticle, ReferenceList};

struct Article {
    id: u64,
    title: String,
    pub_model: String,
    keywords: Vec<String>,
    language: Vec<String>,
    journal: Journal,
    authors: Option<AuthorList>,
    date_created: Date,
    date_completed: Date,
    publication_types: Vec<PublicationType>,
    grants: Option<GrantList>,
    references: Option<ReferenceList>,
    article_ids: Vec<ArticleId>,
}

impl From<PubmedArticle> for Article {
    fn from(value: PubmedArticle) -> Self {
        let pubmed_data = value.pubmed_data;
        let medline_citation = value.medline_citation;
        Article {
            id: medline_citation.pmid.id,
            title: medline_citation.article.article_title,
            pub_model: medline_citation.article.pub_model,
            keywords: vec![],
            language: vec![],
            journal: medline_citation.article.journal,
            authors: medline_citation.article.author_list,
            date_created: medline_citation.date_revised,
            date_completed: medline_citation.date_completed,
            publication_types: medline_citation.article.publication_type_list.publication_type,
            grants: medline_citation.article.grant_list,
            references: pubmed_data.reference_list,
            article_ids: pubmed_data.article_id_list.article_id,
        }
    }
}