impl ::Dispatchable for Call
{
    type Origin = Origin;
    type Config = Call;
    type Info = ::DispatchInfo;
    type PostInfo = PostDispatchInfo;
    fn dispatch (self, origin : Origin) -> dispatch::DispatchResultWithPostInfo {
        if ! <Self::Origin as OriginTrait>::filter_call(&origin, &self) {
          return Result :: Err (frame_system :: Error :: < Runtime > :: CallFiltered . into ()) ;
        }
        UnfilteredDispatchable::dispatch_bypass_filter(self, origin)
    }
}
impl UnfilteredDispatchable for Call
{
    type Origin = Origin;
    fn dispatch_bypass_filter (self , origin: Origin) -> DispatchResultWithPostInfo {
        match self {
          Call :: System (call) => UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) ,
          Call :: SimplePallet (call) => UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) ,
        }
    }
}
