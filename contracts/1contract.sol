contract Vault {
    address public owner;

    mapping(address => uint256) balances;

    function deposit() external payable {}

    function withdraw(uint256 amount) external {}
}