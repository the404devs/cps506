module Cauchy where

    data CauchyList = CauchyList Int [Int]

    instance Eq CauchyList where{
        -- Implement Eq type class here
        (CauchyList p1 c1) == (CauchyList p2 c2) = (p1==p2 && c1==c2);
    }
        
    instance Num CauchyList where
        -- Implement Num type class here
        (CauchyList p1 c1) + (CauchyList p2 c2) = CauchyList (p1) (map (`mod` 31) (zipWith (+) c1 c2))
        (CauchyList p1 c1) - (CauchyList p2 c2) = CauchyList (p1) (map (`mod` 31) (zipWith (-) c1 c2))
        (CauchyList p1 c1) * (Int x) = CauchyList (p1) ([i*x | i <- c1])

        
    instance Show CauchyList where
        -- Implement Show type class here
        show (CauchyList p1 c1) = "p: " ++ (show p1) ++ "\nlength: " ++ (show (length c1)) ++ "\ncontent: " ++ (show c1)



