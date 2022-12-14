use casper_types::ApiError;

#[repr(u16)]
pub enum Errors {
    /// 65,551 for (UniswapV2 Core WCSPR OverFlow1)
    UniswapV2CoreWCSPROverFlow1 = 15,
    /// 65,552 for (UniswapV2 Core WCSPR OverFlow2)
    UniswapV2CoreWCSPROverFlow2 = 16,
    /// 65,553 for (UniswapV2 Core WCSPR OverFlow3)
    UniswapV2CoreWCSPROverFlow3 = 17,

    /// 65,556 for (UniswapV2 Factory No Pair Exists1)
    UniswapV2FactoryNoPairExists1 = 20,
    /// 65,557 for (UniswapV2 Factory No Pair Exists2)
    UniswapV2FactoryNoPairExists2 = 21,
    /// 65,558 for (UniswapV2 Factory WhiteList Pair Mismatch)
    UniswapV2FactoryWhiteListPairMismatch = 22,
    /// 65,559 for (UniswapV2 Factory Zero Address)
    UniswapV2FactoryZeroAddress = 23,
    /// 65,560 for (UniswapV2 Factory Pair Exists1)
    UniswapV2FactoryPairExists1 = 24,
    /// 65,561 for (UniswapV2 Factory Pair Exists2)
    UniswapV2FactoryPairExists2 = 25,
    /// 65,562 for (UniswapV2 Factory Forbidden1)
    UniswapV2FactoryForbidden1 = 26,
    /// 65,563 for (UniswapV2 Factory Forbidden2)
    UniswapV2FactoryForbidden2 = 27,
    /// 65,564 for (UniswapV2 Factory Identical Addresses)
    UniswapV2FactoryIdenticalAddresses = 28,
    /// 65,565 for (UniswapV2 Factory Not In White List)
    UniswapV2FactoryNotInWhiteList = 29,
    /// 65,566 for (UniswapV2 Factory Not Owner)
    UniswapV2FactoryNotOwner = 30,

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
    /// 65,613 for (UniswapV2 Core Pair Not Owner)
    UniswapV2CorePairNotOwner = 77,
    /// 65,614 for (UniswapV2 Core Pair Paused)
    UniswapV2CorePairPaused = 78,
    /// 65,615 for (UniswapV2 Core Cannot Pause)
    UniswapV2CoreCannotPause = 79,
    /// 65,616 for (UniswapV2 Core Cannot Unpause)
    UniswapV2CoreCannotUnpause = 80,
    /// 65,617 for (UniswapV2 Core Pair Locked3)
    UniswapV2CorePairLocked3 = 81,

    /// 65,644 for (UniswapV2 Core FlashSwapper Invalid Contract Address)
    UniswapV2CoreFlashSwapperInvalidContractAddress = 108,
    /// 65,645 for (UniswapV2 Core FlashSwapper UnderFlow)
    UniswapV2CoreFlashSwapperUnderFlow = 109,
    /// 65,646 for (UniswapV2 Core FlashSwapper UnderFlow1)
    UniswapV2CoreFlashSwapperOverFlow1 = 110,
    /// 65,647 for (UniswapV2 Core FlashSwapper UnderFlow2)
    UniswapV2CoreFlashSwapperOverFlow2 = 111,
    /// 65,648 for (UniswapV2 Core FlashSwapper UnderFlow3)
    UniswapV2CoreFlashSwapperOverFlow3 = 112,
    /// 65,649 for (UniswapV2 Core FlashSwapper Amount Too Big)
    UniswapV2CoreFlashSwapperAmountTooBig = 113,
    /// 65,650 for (UniswapV2 Core FlashSwapper Requested Pay Token Is Not Available)
    UniswapV2CoreFlashSwapperRequestedPayTokenIsNotAvailable = 114,
    /// 65,651 for (UniswapV2 Core FlashSwapper Requested Borrow Token Is Not Available)
    UniswapV2CoreFlashSwapperRequestedBorrowTokenIsNotAvailable = 115,
    /// 65,652 for (UniswapV2 Core FlashSwapper Requested Requested Pair Is Not Available)
    UniswapV2CoreFlashSwapperRequestedRequestedPairIsNotAvailable = 116,
    /// 65,653 for (UniswapV2 Core FlashSwapper Zero Address)
    UniswapV2CoreFlashSwapperZeroAddress = 117,
    /// 65,654 for (UniswapV2 Core FlashSwapper Pair Exists)
    UniswapV2CoreFlashSwapperPairExists = 118,
    /// 65,655 for (UniswapV2 Core FlashSwapper Permissioned Pair Access)
    UniswapV2CoreFlashSwapperPermissionedPairAccess = 119,
}

impl From<Errors> for ApiError {
    fn from(error: Errors) -> ApiError {
        ApiError::User(error as u16)
    }
}
