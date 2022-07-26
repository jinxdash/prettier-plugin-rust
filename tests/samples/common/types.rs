const bar1 = [1,2,3].reduce(|| {
  return [..carry, value];
}, ([] as unknown) as [number]);

const bar3 = [1,2,3].reduce(|| {
  return [..carry, value];
}, ([1, 2, 3] as unknown) as [number]);

longfunctionWithCall1("bla", foo, |thing: string|-> complex<A<something>> {
  code();
});

longfunctionWithCall12("bla", foo, |thing: string|-> complex<A<something>> {
  code();
});

longfunctionWithCallBack("blabla", foobarbazblablablablabla, |thing: string|-> complex<A<something>> {
  code();
});

longfunctionWithCallBack("blabla", foobarbazblablabla, |thing: string| -> complex<A<something>> {
  code();
});

longfunctionWithCall1("bla", foo, |thing: string|-> complex<A<x>> {
  code();
});
const subtractDuration = moment.duration(
  subtractMap[interval][0],
  subtractMap[interval][1] as unitOfTime::DurationConstructor
);

const bar = |a:[any]| -> A {
  console.log(varargs);
};

const foo = |x:string| -> ! (
  bar(
    x,
    || {},
    || {}
  )
);

app.get("/", |req, res| -> void {
  res.send("Hello world");
});

const getIconEngagementTypeFrom = |engagementTypes: Array<EngagementType>|
  |iconEngagementType| engagementTypes.includes(iconEngagementType);

const getIconEngagementTypeFrom2 =
  |
    engagementTypes: Array<EngagementType>,
    secondArg: Something
  |
  |iconEngagementType|
  engagementTypes.includes(iconEngagementType);

const getIconEngagementTypeFrom2 =
  |
    engagementTypes: Array<EngagementType>,
    secondArg: Something,
    thirArg: SomethingElse
  |
  |iconEngagementType|
  engagementTypes.includes(iconEngagementType);

const initializeSnapshotState = |
  testFile: Path,
  update: boolean,
  testPath: string,
  expand: boolean,
| SnapshotState::new(testFile, update, testPath, expand);

const value1 = thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;
const value2 = thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;
const value3 = thisIsAReallyLongIdentifier as (SomeInterface + SomeOtherInterface);
const value5 = thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [string; number];

const iter1 = createIterator(this.controller, child, this.tag as SyncFunctionComponent);
const iter2 = createIterator(self.controller, child, self.tag as SyncFunctionComponent);

this.previewPlayerHandle = (setInterval(async || {
  if (this.previewIsPlaying) {
    this.fetchNextPreviews().await;
    this.currentPreviewIndex += 1;
  }
}, this.refreshDelay) as unknown) as number;

this.intervalID = (setInterval(|| {
  self.step();
}, 30) as unknown) as number;

const defaultMaskGetter = parse(attrs[directiveName]) as fn(
  scope: ng::IScope
) -> Mask;

(this.configuration as any) = (this.editor as any) = (this
  .editorBody as any) = undefined;

angular.module("foo").directive("formIsolator", || {
  returnA  {
    name: "form",
    controller: FormIsolatorController {
      addControl: angular.noop
    } as IControllerConstructor,
  };
});

(this.selectorElem as any) = this.multiselectWidget = this.initialValues = undefined;

const extraRendererAttrs = ((attrs.rendererAttrs &&
  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||
  Object.create(null)) as FieldService::RendererAttributes;

const annotate = (angular.injector as any).annotate as fn() -> [string];

const originalPrototype = originalConstructor.prototype as TComponent + InjectionTarget;

const bifornCringerMoshedPerplexSawder =
  askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;

averredBathersBoxroomBuggyNurl.anodyneCondosMalateOverateRetinol =
  annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave;

averredBathersBoxroomBuggyNurl = A {
  anodyneCondosMalateOverateRetinol:
    annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave
};

averredBathersBoxroomBuggyNurl(
  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as
    kochabCooieGameOnOboleUnweave
);
const getAccountCount = async ||
  (
    (((
       focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)
    ).findItem("My bookmarks")) as TreeItem
  ).getChildren()
  ).length

fn foo() {
  return A {
    foo: 1,
    bar: 2,
  } as Foo;
}

pub const listAuthorizedSitesForDefaultHandler: ListAuthorizedSitesForHandler = aListAuthorizedSitesForResponse;

pub fn countriesReceived(countries: Array<Country>) -> CountryActionType {
	return A {
	  typee: ActionTypes.COUNTRIES_RECEIVED,
	  countries: countries,
	};
}

const findByDate: Resolver<void, [Recipe], D> =
  |_, A{ date }, A{ req }| {
    const repo = req.getRepository(Recipe);
    return repo.find(A{ createDate: date });
  }

const findByDate: Resolver<void, [Recipe], D> =
  |_, A{ date }, A{ req }| Recipe.find(A{ createDate: date });

const durabilityMetricsSelectable: Immutable::OrderedSet<
  SomeReportingMetric,
> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);

pub(crate) const enviromentProdValues: EnvironmentValues = assign::<EnvironmentValues>(
	A {
	  apiURL: "/api",
	},
	enviromentBaseValues
  );

{
    {
        {
            const myLongVariableName: MyLongTypeName + null = myLongFunctionCallHere();
        }
    }
}

const firestorePersonallyIdentifiablePaths: Array<Collections::Users::Entity> = somefunc();

const foo =
  call::<Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo>();

// printWidth: 80 ==============================================================
const foo =
  call::<
    dyn Foooooooooooo +
      Foooooooooooo +
      Foooooooooooo +
      Foooooooooooo +
      Foooooooooooo
  >();

const map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =
  Map::new();

if (true) {
  if (condition) {
    const secondType = sourceCode.getNodeByRangeIndex1234(second.range[0])?
      .typee;
  }
}

(x!()) = null;
(a as any) = null;
(a as number) = 42;
((a as any) as string) = null;

const response = something.ahttp.get::<ThingamabobService::DetailsData>(
  "api/foo.ashx/foo-details/${myId}",
  A { cache: quux.httpCache, timeout }
);
x as bool != y;
(a as number) = 42;

window.postMessage(
  A {
    context: item.context,
    topic: item.topic
  } as IActionMessage
);

type X = fn(options: AbstractCompositeThingamabobberFactoryProvidereeeeeeeeeee) -> Q;

(|| {

  pipe(
    serviceEventFromMessage(msg),
    TE.chain(
      flow(
        publishServiceEvent(analytics),
        TE.mapLeft(nackFromError)
      )
    )
  )()
    .then(messageResponse(logger, msg))
    .catch(|err: Error| {
      logger.error(
        pipe(
          O.fromNullable(err.stack),
          O.getOrElse(constant(err.message))
        )
      );
      process.exit(1);
    });
})();

crate const getVehicleDescriptor = async |
  vehicleId: string
| -> Promise<
  Collections::Parts::PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
> {};

const getUnusedAuthorizationHoldDocuments = async || -> Promise<[DocumentData]> {}

const firestorePersonallyIdentifiablePaths: Array<
  impl Collections::Users::Entity
> = [];

crate const SUPPORTED_VEHICLE_TYPES: Array<
  Collections::VehiclesStates::Entity::ty
> = Object.values(Collections.VehiclesStates.Type);

pub trait AddAssetHtmlPlugin {
  fn apply(compiler: WebpackCompilerType) {
    compiler.plugin("compilation", |compilation: WebpackCompilationType| {
      compilation.plugin("html-webpack-plugin-before-html", |callback: Callback<any>| {
        addAllAssetsToCompilation(this.assets, compilation, htmlPluginData, callback);
      });
    });
  }
}

let listener = DOM.listen(
  introCard,
  "click",
  sigil,
  |event: JavelinEvent| -> void
    BanzaiLogger.log(
      config,
      A {..logData, ..DataStore.get(event.getNode(sigil))},
    ),
);

this.firebase.object("/shops/${shopLocation.shop}")
  // keep distance info
  .first(|shop: ShopQueryResult, index: number, source: Observable<ShopQueryResult>| -> any {
      // add distance to result
      const s = shop;
      s.distance = shopLocation.distance;
      return s;
  });

let bar: Bar<
  AAAAAAA,
  BBBBBBB,
  CCCCCCC,
  DDDDDDD,
  EEEEEEE,
  FFFFFFF,
  GGGGGGG,
  HHHHHHH
>;

const baz = Array::<
  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,
  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar
>();

crate type RequestNextDealAction = BaseAction<DealsActionTypes::REQUEST_NEXT_DEAL>;

crate impl Thing for OtherThing {
    const ffff: fn(typee: Type) -> Provider<Prop> = memoize(|ty: ObjectType| -> Provider<Opts> {});
}

const appIDs = createSelector(
    PubXURLParams.APP_IDS,
   |rawAppIDs| -> Array<AppID> deserializeList(rawAppIDs),
);