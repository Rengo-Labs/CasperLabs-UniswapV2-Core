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
    UniswapV2FactoryNoPairExists1 = 18,
    /// 65,557 for (UniswapV2 Factory No Pair Exists2)
    UniswapV2FactoryNoPairExists2 = 19,
    /// 65,558 for (UniswapV2 Factory WhiteList Pair Mismatch)
    UniswapV2FactoryWhiteListPairMismatch = 20,
    /// 65,559 for (UniswapV2 Factory Zero Address1)
    UniswapV2FactoryZeroAddress1 = 21,
    /// 65,559 for (UniswapV2 Factory Zero Address2)
    UniswapV2FactoryZeroAddress2 = 22,
    /// 65,560 for (UniswapV2 Factory Pair Exists1)
    UniswapV2FactoryPairExists1 = 23,
    /// 65,561 for (UniswapV2 Factory Pair Exists2)
    UniswapV2FactoryPairExists2 = 24,
    /// 65,562 for (UniswapV2 Factory Forbidden1)
    UniswapV2FactoryForbidden1 = 25,
    /// 65,563 for (UniswapV2 Factory Forbidden2)
    UniswapV2FactoryForbidden2 = 26,
    /// 65,564 for (UniswapV2 Factory Identical Addresses1)
    UniswapV2FactoryIdenticalAddresses1 = 27,
    /// 65,564 for (UniswapV2 Factory Identical Addresses2)
    UniswapV2FactoryIdenticalAddresses2 = 28,
    /// 65,565 for (UniswapV2 Factory Not In White List)
    UniswapV2FactoryNotInWhiteList1 = 29,
    /// 65,565 for (UniswapV2 Factory Not In White List)
    UniswapV2FactoryNotInWhiteList2 = 30,
    /// 65,566 for (UniswapV2 Factory Not Owner)
    UniswapV2FactoryNotOwner = 31,

    /// 65,567 for (UniswapV2 Core Pair Insufficient Output Amount)
    UniswapV2CorePairInsufficientOutputAmount = 32,
    /// 65,568 for (UniswapV2 Core Pair Insufficient Liquidity)
    UniswapV2CorePairInsufficientLiquidity = 33,
    /// 65,569 for (UniswapV2 Core Pair Invalid To)
    UniswapV2CorePairInvalidTo = 34,
    /// 65,570 for (UniswapV2 Core Pair Insufficient Input Amount)
    UniswapV2CorePairInsufficientInputAmount = 35,
    /// 65,571 for (UniswapV2 Core Pair Insufficient Converted Balance)
    UniswapV2CorePairInsufficientConvertedBalance = 36,
    /// 65,572 for (UniswapV2 Core Pair Insufficient Liquidity Minted)
    UniswapV2CorePairInsufficientLiquidityMinted = 37,
    /// 65,573 for (UniswapV2 Core Pair Insufficient Liquidity Burned)
    UniswapV2CorePairInsufficientLiquidityBurned = 38,
    /// 65,574 for (UniswapV2 Core Pair Denominator Is Zero)
    UniswapV2CorePairDenominatorIsZero = 39,
    /// 65,575 for (UniswapV2 Core Pair Locked1)
    UniswapV2CorePairLocked1 = 40,
    /// 65,576 for (UniswapV2 Core Pair Locked2)
    UniswapV2CorePairLocked2 = 41,
    /// 65,577 for (UniswapV2 Core Pair UnderFlow1)
    UniswapV2CorePairUnderFlow1 = 42,
    /// 65,578 for (UniswapV2 Core Pair UnderFlow2)
    UniswapV2CorePairUnderFlow2 = 43,
    /// 65,579 for (UniswapV2 Core Pair UnderFlow3)
    UniswapV2CorePairUnderFlow3 = 44,
    /// 65,580 for (UniswapV2 Core Pair UnderFlow4)
    UniswapV2CorePairUnderFlow4 = 45,
    /// 65,581 for (UniswapV2 Core Pair UnderFlow5)
    UniswapV2CorePairUnderFlow5 = 46,
    /// 65,582 for (UniswapV2 Core Pair UnderFlow6)
    UniswapV2CorePairUnderFlow6 = 47,
    /// 65,583 for (UniswapV2 Core Pair UnderFlow7)
    UniswapV2CorePairUnderFlow7 = 48,
    /// 65,584 for (UniswapV2 Core Pair UnderFlow8)
    UniswapV2CorePairUnderFlow8 = 49,
    /// 65,585 for (UniswapV2 Core Pair UnderFlow9)
    UniswapV2CorePairUnderFlow9 = 50,
    /// 65,586 for (UniswapV2 Core Pair OverFlow1)
    UniswapV2CorePairOverFlow1 = 51,
    /// 65,587 for (UniswapV2 Core Pair OverFlow2)
    UniswapV2CorePairOverFlow2 = 52,
    /// 65,588 for (UniswapV2 Core Pair OverFlow3)
    UniswapV2CorePairOverFlow3 = 53,
    /// 65,589 for (UniswapV2 Core Pair OverFlow4)
    UniswapV2CorePairOverFlow4 = 54,
    /// 65,590 for (UniswapV2 Core Pair OverFlow5)
    UniswapV2CorePairOverFlow5 = 55,
    /// 65,591 for (UniswapV2 Core Pair OverFlow6)
    UniswapV2CorePairOverFlow6 = 56,
    /// 65,592 for (UniswapV2 Core Pair Multiplication OverFlow1)
    UniswapV2CorePairMultiplicationOverFlow1 = 57,
    /// 65,593 for (UniswapV2 Core Pair Multiplication OverFlow2)
    UniswapV2CorePairMultiplicationOverFlow2 = 58,
    /// 65,594 for (UniswapV2 Core Pair Multiplication OverFlow3)
    UniswapV2CorePairMultiplicationOverFlow3 = 59,
    /// 65,595 for (UniswapV2 Core Pair Multiplication OverFlow4)
    UniswapV2CorePairMultiplicationOverFlow4 = 60,
    /// 65,596 for (UniswapV2 Core Pair Multiplication OverFlow5)
    UniswapV2CorePairMultiplicationOverFlow5 = 61,
    /// 65,597 for (UniswapV2 Core Pair Multiplication OverFlow6)
    UniswapV2CorePairMultiplicationOverFlow6 = 62,
    /// 65,598 for (UniswapV2 Core Pair Multiplication OverFlow7)
    UniswapV2CorePairMultiplicationOverFlow7 = 63,
    /// 65,599 for (UniswapV2 Core Pair Multiplication OverFlow8)
    UniswapV2CorePairMultiplicationOverFlow8 = 64,
    /// 65,600 for (UniswapV2 Core Pair Multiplication OverFlow9)
    UniswapV2CorePairMultiplicationOverFlow9 = 65,
    /// 65,601 for (UniswapV2 Core Pair Multiplication OverFlow10)
    UniswapV2CorePairMultiplicationOverFlow10 = 66,
    /// 65,602 for (UniswapV2 Core Pair Multiplication OverFlow11)
    UniswapV2CorePairMultiplicationOverFlow11 = 67,
    /// 65,603 for (UniswapV2 Core Pair Multiplication OverFlow12)
    UniswapV2CorePairMultiplicationOverFlow12 = 68,
    /// 65,604 for (UniswapV2 Core Pair Multiplication OverFlow13)
    UniswapV2CorePairMultiplicationOverFlow13 = 69,
    /// 65,605 for (UniswapV2 Core Pair Multiplication OverFlow14)
    UniswapV2CorePairMultiplicationOverFlow14 = 70,
    /// 65,606 for (UniswapV2 Core Pair Multiplication OverFlow15)
    UniswapV2CorePairMultiplicationOverFlow15 = 71,
    /// 65,607 for (UniswapV2 Core Pair Multiplication OverFlow16)
    UniswapV2CorePairMultiplicationOverFlow16 = 72,
    /// 65,608 for (UniswapV2 Core Pair Multiplication OverFlow17)
    UniswapV2CorePairMultiplicationOverFlow17 = 73,
    /// 65,609 for (UniswapV2 Core Pair Division OverFlow1)
    UniswapV2CorePairDivisionOverFlow1 = 74,
    /// 65,610 for (UniswapV2 Core Pair Division OverFlow2)
    UniswapV2CorePairDivisionOverFlow2 = 75,
    /// 65,611 for (UniswapV2 Core Pair Division OverFlow3)
    UniswapV2CorePairDivisionOverFlow3 = 76,
    /// 65,612 for (UniswapV2 Core Pair Forbidden1)
    UniswapV2CorePairForbidden1 = 77,
    /// 65,612 for (UniswapV2 Core Pair Forbidden2)
    UniswapV2CorePairForbidden2 = 78,
    /// 65,613 for (UniswapV2 Core Pair Not Owner)
    UniswapV2CorePairNotOwner = 79,
    /// 65,614 for (UniswapV2 Core Pair Paused)
    UniswapV2CorePairPaused = 80,
    /// 65,615 for (UniswapV2 Core Cannot Pause)
    UniswapV2CoreCannotPause = 81,
    /// 65,616 for (UniswapV2 Core Cannot Unpause)
    UniswapV2CoreCannotUnpause = 82,
    /// 65,617 for (UniswapV2 Core Pair Locked3)
    UniswapV2CorePairLocked3 = 83,

    /// 65,644 for (UniswapV2 Core FlashSwapper Invalid Contract Address)
    UniswapV2CoreFlashSwapperInvalidContractAddress = 84,
    /// 65,645 for (UniswapV2 Core FlashSwapper UnderFlow)
    UniswapV2CoreFlashSwapperUnderFlow = 85,
    /// 65,646 for (UniswapV2 Core FlashSwapper UnderFlow1)
    UniswapV2CoreFlashSwapperOverFlow1 = 86,
    /// 65,647 for (UniswapV2 Core FlashSwapper UnderFlow2)
    UniswapV2CoreFlashSwapperOverFlow2 = 87,
    /// 65,648 for (UniswapV2 Core FlashSwapper UnderFlow3)
    UniswapV2CoreFlashSwapperOverFlow3 = 88,
    /// 65,649 for (UniswapV2 Core FlashSwapper Amount Too Big)
    UniswapV2CoreFlashSwapperAmountTooBig = 89,
    /// 65,650 for (UniswapV2 Core FlashSwapper Requested Pay Token Is Not Available)
    UniswapV2CoreFlashSwapperRequestedPayTokenIsNotAvailable = 90,
    /// 65,651 for (UniswapV2 Core FlashSwapper Requested Borrow Token Is Not Available)
    UniswapV2CoreFlashSwapperRequestedBorrowTokenIsNotAvailable = 91,
    /// 65,652 for (UniswapV2 Core FlashSwapper Requested Requested Pair Is Not Available)
    UniswapV2CoreFlashSwapperRequestedRequestedPairIsNotAvailable = 92,
    /// 65,653 for (UniswapV2 Core FlashSwapper Zero Address)
    UniswapV2CoreFlashSwapperZeroAddress = 93,
    /// 65,655 for (UniswapV2 Core FlashSwapper Permissioned Pair Access)
    UniswapV2CoreFlashSwapperPermissionedPairAccess = 94,
    /// 65,655 for (UniswapV2 Core Erc20 Secure Only Admin)
    UniswapV2CoreErc20SecureOnlyAdmin1 = 95,
    /// 65,655 for (UniswapV2 Core Erc20 Secure Only Admin)
    UniswapV2CoreErc20SecureOnlyAdmin2 = 96,
}

impl From<Errors> for ApiError {
    fn from(error: Errors) -> ApiError {
        ApiError::User(error as u16)
    }
}
