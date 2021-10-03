namespace N {
template <typename> class B{};
template <typename c> class C {
public:
  using U = B<c>;
};
}
class A : N::C<A> {
  U u;
};
