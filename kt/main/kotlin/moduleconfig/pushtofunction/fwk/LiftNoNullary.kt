package tryout.moduleconfig.pushtovar.fwk

fun <S, T> liftToNullary(f: (S) -> T): ((() -> S)?) -> () -> T {
    return { sSrc ->
        {
            if (sSrc == null) {
                throw ConfigurationException("Null configuration source")
            }
            f(sSrc())
        }
    }
}
