const DEFAULT_PER_PAGE: u64 = 20;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Paginate {
    pub page: u64,
    pub per_page: u64,
}

impl Default for Paginate {
    fn default() -> Self {
        Paginate::new(DEFAULT_PER_PAGE)
    }
}

impl Paginate {
    pub fn new(per_page: u64) -> Self {
        Paginate {
            page: 1,
            per_page,
        }
    }

    pub fn validated(mut self) -> Self {
        if self.per_page == 0 {
            self.per_page = DEFAULT_PER_PAGE;
        }

        if self.page == 0 {
            self.page = 1;
        }

        self
    }

    pub fn skip(&self) -> u64 {
        self.per_page * (self.page - 1)
    }

    pub fn take(&self) -> u64 {
        self.per_page
    }

    pub fn into_pagination(self, total: u64) -> Pagination {
        if total > 0 {
            let last_page = (total as f64 / self.per_page as f64).ceil() as u64;
            Pagination {
                paginate: self,
                total,
                last_page,
            }
        } else {
            Pagination {
                paginate: Paginate::new(self.per_page),
                ..Pagination::default()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    #[serde(flatten)]
    paginate: Paginate,
    total: u64,
    last_page: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            paginate: Paginate::default(),
            total: 0,
            last_page: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginated<T> {
    data: Vec<T>,
    #[serde(flatten)]
    pagination: Option<Pagination>,
}

pub trait IntoPaginated<T> {
    fn raw_into(self, paginate: Option<Paginate>) -> Paginated<T>;
}

impl<T> IntoPaginated<T> for Vec<(T, i64)> {
    fn raw_into(self, paginate: Option<Paginate>) -> Paginated<T> {
        let total = self.first()
            .map(|(_, total)| *total as u64)
            .unwrap_or(0);

        Paginated {
            data: self.into_iter().map(|r| r.0).collect(),
            pagination: paginate.map(|p| p.into_pagination(total)),
        }
    }
}