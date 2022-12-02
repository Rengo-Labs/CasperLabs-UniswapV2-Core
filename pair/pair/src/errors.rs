use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    /// 65,567 for (UniswapV2 Core Pair Insufficient Output Amount)
    UniswapV2CorePairInsufficientOutputAmount = 31,
    /// 65,568 for (UniswapV2 Core Pair Insufficient Liquidity)
    UniswapV2CorePairInsufficientLiquidity = 32,
    /// 65,569 for (UniswapV2 Core Pair Invalid To)
    UniswapV2CorePairInvalidTo = 33,
    /// 65,570 for (UniswapV2 Core Pair Insufficient Input Amount)
    UniswapV2CorePairInsufficientInputAmount = 34,
    /// 65,571 for (UniswapV2 Core Pair Insufficient Converted Balance)
    UniswapV2CorePairInsufficientConvertedBalance = 35,
    /// 65,572 for (UniswapV2 Core Pair Insufficient Liquidity Minted)
    UniswapV2CorePairInsufficientLiquidityMinted = 36,
    /// 65,573 for (UniswapV2 Core Pair Insufficient Liquidity Burned)
    UniswapV2CorePairInsufficientLiquidityBurned = 37,
    /// 65,574 for (UniswapV2 Core Pair Denominator Is Zero)
    UniswapV2CorePairDenominatorIsZero = 38,
    /// 65,575 for (UniswapV2 Core Pair Locked1)
    UniswapV2CorePairLocked1 = 39,
    /// 65,576 for (UniswapV2 Core Pair Locked2)
    UniswapV2CorePairLocked2 = 40,
    /// 65,577 for (UniswapV2 Core Pair UnderFlow1)
    UniswapV2CorePairUnderFlow1 = 41,
    /// 65,578 for (UniswapV2 Core Pair UnderFlow2)
    UniswapV2CorePairUnderFlow2 = 42,
    /// 65,579 for (UniswapV2 Core Pair UnderFlow3)
    UniswapV2CorePairUnderFlow3 = 43,
    /// 65,580 for (UniswapV2 Core Pair UnderFlow4)
    UniswapV2CorePairUnderFlow4 = 44,
    /// 65,581 for (UniswapV2 Core Pair UnderFlow5)
    UniswapV2CorePairUnderFlow5 = 45,
    /// 65,582 for (UniswapV2 Core Pair UnderFlow6)
    UniswapV2CorePairUnderFlow6 = 46,
    /// 65,583 for (UniswapV2 Core Pair UnderFlow7)
    UniswapV2CorePairUnderFlow7 = 47,
    /// 65,584 for (UniswapV2 Core Pair UnderFlow8)
    UniswapV2CorePairUnderFlow8 = 48,
    /// 65,585 for (UniswapV2 Core Pair UnderFlow9)
    UniswapV2CorePairUnderFlow9 = 49,
    /// 65,586 for (UniswapV2 Core Pair OverFlow1)
    UniswapV2CorePairOverFlow1 = 50,
    /// 65,587 for (UniswapV2 Core Pair OverFlow2)
    UniswapV2CorePairOverFlow2 = 51,
    /// 65,588 for (UniswapV2 Core Pair OverFlow3)
    UniswapV2CorePairOverFlow3 = 52,
    /// 65,589 for (UniswapV2 Core Pair OverFlow4)
    UniswapV2CorePairOverFlow4 = 53,
    /// 65,590 for (UniswapV2 Core Pair OverFlow5)
    UniswapV2CorePairOverFlow5 = 54,
    /// 65,591 for (UniswapV2 Core Pair OverFlow6)
    UniswapV2CorePairOverFlow6 = 55,
    /// 65,592 for (UniswapV2 Core Pair Multiplication OverFlow1)
    UniswapV2CorePairMultiplicationOverFlow1 = 56,
    /// 65,593 for (UniswapV2 Core Pair Multiplication OverFlow2)
    UniswapV2CorePairMultiplicationOverFlow2 = 57,
    /// 65,594 for (UniswapV2 Core Pair Multiplication OverFlow3)
    UniswapV2CorePairMultiplicationOverFlow3 = 58,
    /// 65,595 for (UniswapV2 Core Pair Multiplication OverFlow4)
    UniswapV2CorePairMultiplicationOverFlow4 = 59,
    /// 65,596 for (UniswapV2 Core Pair Multiplication OverFlow5)
    UniswapV2CorePairMultiplicationOverFlow5 = 60,
    /// 65,597 for (UniswapV2 Core Pair Multiplication OverFlow6)
    UniswapV2CorePairMultiplicationOverFlow6 = 61,
    /// 65,598 for (UniswapV2 Core Pair Multiplication OverFlow7)
    UniswapV2CorePairMultiplicationOverFlow7 = 62,
    /// 65,599 for (UniswapV2 Core Pair Multiplication OverFlow8)
    UniswapV2CorePairMultiplicationOverFlow8 = 63,
    /// 65,600 for (UniswapV2 Core Pair Multiplication OverFlow9)
    UniswapV2CorePairMultiplicationOverFlow9 = 64,
    /// 65,601 for (UniswapV2 Core Pair Multiplication OverFlow10)
    UniswapV2CorePairMultiplicationOverFlow10 = 65,
    /// 65,602 for (UniswapV2 Core Pair Multiplication OverFlow11)
    UniswapV2CorePairMultiplicationOverFlow11 = 66,
    /// 65,603 for (UniswapV2 Core Pair Multiplication OverFlow12)
    UniswapV2CorePairMultiplicationOverFlow12 = 67,
    /// 65,604 for (UniswapV2 Core Pair Multiplication OverFlow13)
    UniswapV2CorePairMultiplicationOverFlow13 = 68,
    /// 65,605 for (UniswapV2 Core Pair Multiplication OverFlow14)
    UniswapV2CorePairMultiplicationOverFlow14 = 69,
    /// 65,606 for (UniswapV2 Core Pair Multiplication OverFlow15)
    UniswapV2CorePairMultiplicationOverFlow15 = 70,
    /// 65,607 for (UniswapV2 Core Pair Multiplication OverFlow16)
    UniswapV2CorePairMultiplicationOverFlow16 = 71,
    /// 65,608 for (UniswapV2 Core Pair Multiplication OverFlow17)
    UniswapV2CorePairMultiplicationOverFlow17 = 72,
    /// 65,609 for (UniswapV2 Core Pair Division OverFlow1)
    UniswapV2CorePairDivisionOverFlow1 = 73,
    /// 65,610 for (UniswapV2 Core Pair Division OverFlow2)
    UniswapV2CorePairDivisionOverFlow2 = 74,
    /// 65,611 for (UniswapV2 Core Pair Division OverFlow3)
    UniswapV2CorePairDivisionOverFlow3 = 75,
    /// 65,612 for (UniswapV2 Core Pair Forbidden)
    UniswapV2CorePairForbidden = 76,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
