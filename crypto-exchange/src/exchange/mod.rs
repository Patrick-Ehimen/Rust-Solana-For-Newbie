pub struct Exchange {
    orders: Vec<Order>,
    trades: Vec<Trade>,
}

impl Exchange {
    pub fn new() -> Self {
        Exchange {
            orders: Vec::new(),
            trades: Vec::new(),
        }
    }

    pub fn execute_trade(&mut self, order: Order) -> Result<Trade, String> {
        // Logic to execute a trade based on the order
        // This is a placeholder for actual implementation
        let trade = Trade::new(order);
        self.trades.push(trade.clone());
        Ok(trade)
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }

    pub fn get_trades(&self) -> &Vec<Trade> {
        &self.trades
    }
}