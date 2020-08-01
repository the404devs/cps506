module Test where
    data Pt = Pt3 Float Float Float
            | Pt2 Float Float
            deriving (Show)

    instance Num Pt where
        (Pt2 x1 y1) + (Pt2 x2 y2) = Pt2 (x1+x2) (y1+y2)