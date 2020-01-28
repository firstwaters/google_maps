use crate::distance_matrix::request::Request;
use time::PrimitiveDateTime;

impl Request {

    /// Specifies the desired arrival time.
    ///
    /// ## Arguments:
    ///
    /// * `arrival_time` ‧ The time the passenger should arrive at their final
    /// destination by.
    ///
    /// ## Description:
    ///
    /// Specifies the desired time of arrival for _transit_ distances. You can
    /// use either the `.with_departure_time()` or the `.with_arrival_time()`
    /// method, but not both together.
    ///
    /// ## Example:
    ///
    /// * Arriving by January 1, 2019 at 12:00:00 AM:
    /// ```
    /// .with_arrival_time(PrimitiveDateTime::new(
    ///     PrimitiveDateTime::new(Date::try_from_ymd(2019, 1, 1).unwrap(),
    ///     Time::midnight()
    /// ))
    /// ```

    pub fn with_arrival_time(&mut self, arrival_time: PrimitiveDateTime) -> &mut Request {
        self.arrival_time = Some(arrival_time);
        self
    } // fn

} // impl