// The timid solver fails whenever it has to make a decision. The goal of this solver is to be able to 
// make a single move, and then continue onwards. If the continuation leads to a contradiction, then 
// explore the other possibilities for that move. Similarly, if the single risky move leads to another risky move, 
// we need to be able to continue exploring. 
// 
// Constraint: If we choose, at any point, to not make some move, then we can by definition never make that move again.
// E.g. when we choose to make a red end go up instead of left, we can never place a red cell where we chose to not place it,
// because it would then be 