[
	matches!(1 + 1,   Some(_)                     ),
	matches!(1 + 1,   Some(_) | None if 1 + 1 == 2),
	matches!(1 + 1, | Some(_) | None if 1 + 1 == 2)
]