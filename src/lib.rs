//! This library is not affiliated with the NHS, Public Health England, or the UK Government. This is an unofficial project to provide Rust bindings for the NHS COVID-19 API.
//!
//! This library provides interfaces with the NHS 'Coronavirus (COVID-19) in the UK' data APIs, provided by [gov.uk](https://coronavirus.data.gov.uk).
//! 
//! # Examples
//! 
//! ```
//! use covid19_uk_rs;
//! 
//! let mut req = covid19_uk_rs::Request::new(covid19_uk_rs::AreaType::Nation, covid19_uk_rs::Metric::CumulativeCasesByPublishDate(0));
//! req.add_filter(covid19_uk_rs::Filter::new(covid19_uk_rs::FilterValue::AreaName(String::from("england"))));
//! 
//! for day in req.get().unwrap() {
//!     match day.get(0) {
//!         Some(i) => match i {
//!             covid19_uk_rs::Metric::CumulativeCasesByPublishDate(i) => println!("{}", i),
//!             _ => {},
//!         },
//!         None => {},
//!     }
//! }
//! ```
use reqwest;
use time::Date;

const API_URL: &str = "https://api.coronavirus.data.gov.uk/v1/data";

#[derive(Debug)]
pub enum Error {
    RequestErr(reqwest::Error),
    NoData,
    TooManyRequests,
}

#[derive(Debug)]
pub enum AreaType {
    Overview,
    Nation,
    Region,
    NHSRegion,
    UTLA,
    LTLA,
}

/// Valid filter types and their associated value for specific data requests
#[derive(Debug)]
pub enum FilterValue {
    AreaType(AreaType),
    /// AreaName's attached string must be lowercase.
    AreaName(String),
    AreaCode(String),
    Date(Date),
}

#[derive(Debug)]
pub struct Filter {
    metric: String,
    value: FilterValue,
}
impl Filter {
    pub fn new(value: FilterValue) -> Filter {
        let metric = match value {
            FilterValue::AreaType(_) => String::from("areaType"),
            FilterValue::AreaName(_) => String::from("areaName"),
            FilterValue::AreaCode(_) => String::from("areaCode"),
            FilterValue::Date(_) => String::from("date"),
        };

        Filter { metric, value }
    }
}

/// Valid metrics which may be requested from the NHS API.
#[derive(Debug)]
pub enum Metric {
    AreaType(AreaType),
    AreaName(String),
    AreaCode(String),
    Date(Date),
    Hash(String),
    NewCasesByPublishDate(i32),
    CumulativeCasesByPublishDate(i32),
    CumulativeCasesBySpecimenDateRange(i32),
    NewCasesBySpecimenDate(i32),
    MaleCases(i32),
    FemaleCases(i32),
    NewPillarOneTestsByPublishDate(i32),
    CumulativePillarOneTestsByPublishDate(i32),
    NewPillarTwoTestsByPublishDate(i32),
    CumulativePillarTwoTestsByPublishDate(i32),
    NewPillarThreeTestsByPublishDate(i32),
    CumulativePillarThreeTestsByPublishDate(i32),
    NewPillarFourTestsByPublishDate(i32),
    CumulativePillarFourTestsByPublishDate(i32),
    NewAdmissions(i32),
    CumulativeAdmissions(i32),
    CumulativeAdmissionsByAge(i32),
    CumulativeTestsByPublishDate(i32),
    NewTestsByPublishDate(i32),
    CovidOccupiedMechanicalVentilatorBeds(i32),
    HospitalCases(i32),
    PlannedCapacityByPublishDate(i32),
    NewDeathsWithin28DaysByPublishDate(i32),
    CumulativeDeathsWithin28DaysByPublishDate(i32),
}
fn metric_to_str(metric: &Metric) -> &'static str {
    match metric {
        Metric::AreaCode(_) => "areaCode",
        Metric::AreaName(_) => "areaName",
        Metric::AreaType(_) => "areaType",
        Metric::CovidOccupiedMechanicalVentilatorBeds(_) => "covidOccupiedMVBeds",
        Metric::CumulativeAdmissions(_) => "cumAdmissions",
        Metric::CumulativeAdmissionsByAge(_) => "cumAdmissionsByAge",
        Metric::CumulativeCasesByPublishDate(_) => "cumCasesByPublishDate",
        Metric::CumulativeCasesBySpecimenDateRange(_) => "cumCasesBySpecimenDateRange",
        Metric::CumulativeDeathsWithin28DaysByPublishDate(_) => "cumDeaths28DaysByPublishDate",
        Metric::CumulativePillarOneTestsByPublishDate(_) => "cumPillarOneTestsByPublishDate",
        Metric::CumulativePillarTwoTestsByPublishDate(_) => "cumPillarTwoTestsByPublishDate",
        Metric::CumulativePillarThreeTestsByPublishDate(_) => "cumPillarThreeTestsByPublishDate",
        Metric::CumulativePillarFourTestsByPublishDate(_) => "cumPillarFourTestsByPublishDate",
        Metric::CumulativeTestsByPublishDate(_) => "cumTestsByPublishDate",
        Metric::Date(_) => "date",
        Metric::FemaleCases(_) => "femaleCases",
        Metric::Hash(_) => "hash",
        Metric::HospitalCases(_) => "hospitalCases",
        Metric::MaleCases(_) => "maleCases",
        Metric::NewAdmissions(_) => "newAdmissions",
        Metric::NewCasesByPublishDate(_) => "newCasesByPublishDate",
        Metric::NewCasesBySpecimenDate(_) => "newCasesBySpecimenDate",
        Metric::NewDeathsWithin28DaysByPublishDate(_) => "newDeaths28DaysByPublishDate",
        Metric::NewPillarOneTestsByPublishDate(_) => "newPillarOneTestsByPublishDate",
        Metric::NewPillarTwoTestsByPublishDate(_) => "newPillarTwoTestsByPublishDate",
        Metric::NewPillarThreeTestsByPublishDate(_) => "newPillarThreeTestsByPublishDate",
        Metric::NewPillarFourTestsByPublishDate(_) => "newPillarFourTestsByPublishDate",
        Metric::NewTestsByPublishDate(_) => "newTestsByPublishDate",
        Metric::PlannedCapacityByPublishDate(_) => "plannedCapacityByPublishDate",
    }
}

/// The data for the requested metrics for a specific day.
pub type Datum = Vec<Metric>;
/// The complete collection of days.
pub type Data = Vec<Datum>;

/// A request to the API.
///
/// A request is constructed and then submitted to the API. The request may be re-used and modified, if desired, but filters and metrics cannot be removed.
/// 
/// When a request is executed using `get` or `get_latest_by_metric`, a `Data` object is returned, which is a vector of `Datum` elements (these being vectors of `Metric` elements). Each `Datum` represents a specific day's data, with the encompassed `Metric`s storing the result data. The days are returned in the order the API provides (reverse-chronological).
#[derive(Debug)]
pub struct Request {
    filters: Vec<Filter>,
    metrics: Vec<Metric>,
}
impl Request {
    pub fn new(area_type: AreaType, metric: Metric) -> Request {
        Request {
            filters: vec![Filter::new(FilterValue::AreaType(area_type))],
            metrics: vec![metric],
        }
    }

    pub fn add_filter(&mut self, filter: Filter) {
        self.filters.push(filter);
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn get(&self) -> Result<Data, Error> {
        Ok(self.execute(Option::None)?)
    }

    pub fn get_latest_by_metric(&self, metric: Metric) -> Result<Data, Error> {
        Ok(self.execute(Option::Some(metric))?)
    }

    fn execute(&self, latest_by: Option<Metric>) -> Result<Data, Error> {
        let client = reqwest::blocking::Client::new();

        let mut data = vec![];
        let mut page = 1;

        loop {
            let url = self.construct_url(&latest_by, &page);

            let req = client.get(&url)
                            .header("Accepts", "application/json; application/xml; text/csv; application/vnd.PHE-COVID19.v1+json; application/vnd.PHE-COVID19.v1+xml")
                            .header("Content-Type", "application/json");
            
            let res = match req.send() {
                Ok(r) => r,
                Err(e) => return Result::Err(Error::RequestErr(e)),
            };
            let status_code = res.status().as_u16();
            if status_code != 200 {
                if status_code == 204 {
                    return Result::Err(Error::NoData);
                } else if status_code == 429 {
                    return Result::Err(Error::TooManyRequests);
                } else {
                    panic!("Error response from API ({}): {}", status_code, res.text().unwrap_or(String::from("No response text")));
                }
            };

            let body = res.text().unwrap();

            // TODO: parse body data into json (json-rust?), then place into `data`. check if there's a next page, and if there isn't (it's null), then break the loop.
            let resp = match json::parse(&body) {
                Ok(s) => s,
                Err(e) => panic!("Error parsing JSON: {} (body: {})", e, body),
            };

            for day in resp["data"].members() {
                let mut i = 0;
                let mut datum = vec![];
                for metric in &self.metrics {
                    let m = match metric {
                        Metric::AreaCode(_) => Metric::AreaCode(day[i].to_string()),
                        Metric::AreaName(_) => Metric::AreaName(day[i].to_string()),
                        Metric::AreaType(_) => Metric::AreaType(match day[i].to_string().as_str() {
                            "overview" => AreaType::Overview,
                            "nation" => AreaType::Nation,
                            "region" => AreaType::Region,
                            "nhsRegion" => AreaType::NHSRegion,
                            "utla" => AreaType::UTLA,
                            "ltla" => AreaType::LTLA,
                            s => panic!("Unknown area type ({}) provided by API. This likely means the API is a different version and probably incompatible.", s),
                        }),
                        Metric::CovidOccupiedMechanicalVentilatorBeds(_) => Metric::CovidOccupiedMechanicalVentilatorBeds(day[i].as_i32().unwrap()),
                        Metric::CumulativeAdmissions(_) => Metric::CumulativeAdmissions(day[i].as_i32().unwrap()),
                        Metric::CumulativeAdmissionsByAge(_) => Metric::CumulativeAdmissionsByAge(day[i].as_i32().unwrap()),
                        Metric::CumulativeCasesByPublishDate(_) => Metric::CumulativeCasesByPublishDate(day[i].as_i32().unwrap()),
                        Metric::CumulativeCasesBySpecimenDateRange(_) => Metric::CumulativeCasesBySpecimenDateRange(day[i].as_i32().unwrap()),
                        Metric::CumulativeDeathsWithin28DaysByPublishDate(_) => Metric::CumulativeDeathsWithin28DaysByPublishDate(day[i].as_i32().unwrap()),
                        Metric::CumulativePillarOneTestsByPublishDate(_) => Metric::CumulativePillarOneTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::CumulativePillarTwoTestsByPublishDate(_) => Metric::CumulativePillarTwoTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::CumulativePillarThreeTestsByPublishDate(_) => Metric::CumulativePillarThreeTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::CumulativePillarFourTestsByPublishDate(_) => Metric::CumulativePillarFourTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::CumulativeTestsByPublishDate(_) => Metric::CumulativeTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::Date(_) => Metric::Date(Date::parse(day[i].to_string(), "%F").unwrap()),
                        Metric::FemaleCases(_) => Metric::FemaleCases(day[i].as_i32().unwrap()),
                        Metric::Hash(_) => Metric::Hash(day[i].to_string()),
                        Metric::HospitalCases(_) => Metric::HospitalCases(day[i].as_i32().unwrap()),
                        Metric::MaleCases(_) => Metric::MaleCases(day[i].as_i32().unwrap()),
                        Metric::NewAdmissions(_) => Metric::NewAdmissions(day[i].as_i32().unwrap()),
                        Metric::NewCasesByPublishDate(_) => Metric::NewCasesByPublishDate(day[i].as_i32().unwrap()),
                        Metric::NewCasesBySpecimenDate(_) => Metric::NewCasesBySpecimenDate(day[i].as_i32().unwrap()),
                        Metric::NewDeathsWithin28DaysByPublishDate(_) => Metric::NewDeathsWithin28DaysByPublishDate(day[i].as_i32().unwrap()),
                        Metric::NewPillarOneTestsByPublishDate(_) => Metric::NewPillarOneTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::NewPillarTwoTestsByPublishDate(_) => Metric::NewPillarTwoTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::NewPillarThreeTestsByPublishDate(_) => Metric::NewPillarThreeTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::NewPillarFourTestsByPublishDate(_) => Metric::NewPillarFourTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::NewTestsByPublishDate(_) => Metric::NewTestsByPublishDate(day[i].as_i32().unwrap()),
                        Metric::PlannedCapacityByPublishDate(_) => Metric::PlannedCapacityByPublishDate(day[i].as_i32().unwrap()),
                    };
                    datum.push(m);
                }
                data.push(datum);
                i += 1;
            }

            if resp["pagination"]["next"].is_null() {
                break
            } else {
                page += 1
            }
        }
        

        println!("{:#?}", data);

        Ok(data)
    }

    fn construct_url(&self, latest_by: &Option<Metric>, page: &u32) -> String {
        let mut url = String::from(API_URL);
        url.push_str(format!("?filters={}&structure=[{}]&format=json&page={}", self.filters_str(), self.metrics_str(), page).as_str());
        
        if let Option::Some(m) = latest_by {
            url.push_str(format!("&latestBy={}", metric_to_str(m)).as_str());
        }

        println!("URL: {:#?}", url);

        url
    }

    fn filters_str(&self) -> String {
        let mut pairs = String::new();

        let mut multiple_filters = false;
        for filter in &self.filters {
            // if we're past the first filter, we should add a semi-colon before appending our new filter.
            if multiple_filters {
                pairs.push(';');
            }
            multiple_filters = true;

            let value = match &filter.value {
                FilterValue::AreaType(t) => match t {
                    AreaType::Overview => String::from("overview"),
                    AreaType::Nation => String::from("nation"),
                    AreaType::Region => String::from("region"),
                    AreaType::NHSRegion => String::from("nhsRegion"),
                    AreaType::UTLA => String::from("utla"),
                    AreaType::LTLA => String::from("ltla"),
                },
                FilterValue::AreaName(n) => n.to_string(),
                FilterValue::AreaCode(c) => c.to_string(),
                FilterValue::Date(d) => d.format("%Y-%m-%d"),
            };

            pairs.push_str(format!("{}={}", filter.metric, value).as_str());
        };

        pairs
    }

    fn metrics_str(&self) -> String {
        let mut s = String::new();

        let mut multiple_metrics = false;
        for metric in &self.metrics {
            if multiple_metrics {
                s.push_str(", ");
            }
            multiple_metrics = true;

            s.push_str(format!("%22{}%22", metric_to_str(metric)).as_str());
        };

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_request() {
        let mut req = Request::new(AreaType::Nation, Metric::CumulativeCasesByPublishDate(0));
        req.add_filter(Filter::new(FilterValue::AreaName(String::from("england"))));
        req.get().unwrap();
    }
}
