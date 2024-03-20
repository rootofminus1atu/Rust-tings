ALTER PROC MasterDelete
     @ECustomerID INT
AS

-- constants
DECLARE
	@AMOUNT_OF_YEARS INT = 2,
	@HIGH_NET_VALUE MONEY = 500000

-- variables
DECLARE 
	@ICreditLimit INT,
	@IAmountOfOrders INT


-- getting the CreditLIMIT (CreditValue is all NULL)
SELECT @ICreditLimit = c.CreditLimit 
FROM CustomerTBL AS c
WHERE CustomerID = @ECustomerID;

-- getting the amount of orders during the last @AMOUNT_OF_YEARS
SELECT @IAmountOfOrders = COUNT(*) FROM CustomerTBL AS c
INNER JOIN OrderTBL AS o
ON o.CustomerID = c.CustomerID
WHERE c.CustomerID = @ECustomerID
AND DATEDIFF(Y, GETDATE(), o.OrderDate) < @AMOUNT_OF_YEARS


-- checks
IF @ICreditLimit > @HIGH_NET_VALUE
BEGIN
	;THROW 50001, 'cannot deletee a customer withh a high net value', 1
END

IF @IAmountOfOrders > 0 
BEGIN
	DECLARE @ErrMessage VARCHAR(100) = FORMATMESSAGE(N'this customer still has orders from %i years ago', @AMOUNT_OF_YEARS)
	;THROW 50002, @ErrMessage, 1
END


-- deleting the OrderDetails
BEGIN TRY
	DELETE FROM OrderDetailsTBL
	WHERE OrderNo IN (
		-- we use IN because this select returns a collection, very cool
		SELECT OrderNo FROM OrderDetailsTBL AS od
		INNER JOIN OrderTBL AS o
		ON od.OrderNo = o.OrderNo
		WHERE CustomerID =  @ECustomerID
	)
END TRY
BEGIN CATCH
;THROW
END CATCH

-- deleting the order
BEGIN TRY
	DELETE FROM OrderTBL
	WHERE CustomerID = @ECustomerID
END TRY
BEGIN CATCH
;THROW
END CATCH

-- finally deleting the customer itself
BEGIN TRY
	DELETE FROM CustomerTBL
	WHERE CustomerID = @ECustomerID
END TRY
BEGIN CATCH
;THROW
END CATCH