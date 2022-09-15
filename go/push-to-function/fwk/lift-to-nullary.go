package fwk

func LiftToNullary[S, T any](f func(S) T) func(func() S) func() T {
	return func(sSrc func() S) func() T {
		return func() T {
			return f(sSrc())
		}
	}
}
