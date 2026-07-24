// Current score of the party
struct Score {
    odd: u8,
    even: u8,
}

impl Score {
    fn new() -> Self {
        Score { odd: 0, even: 0 }
    }
}
