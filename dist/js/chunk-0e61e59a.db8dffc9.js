(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-0e61e59a"],{"08ed":function(t,e,a){},"0fea":function(t,e,a){"use strict";a.d(e,"g",(function(){return s})),a.d(e,"e",(function(){return o})),a.d(e,"h",(function(){return d})),a.d(e,"f",(function(){return r})),a.d(e,"m",(function(){return l})),a.d(e,"k",(function(){return u})),a.d(e,"i",(function(){return c})),a.d(e,"l",(function(){return _})),a.d(e,"j",(function(){return p})),a.d(e,"p",(function(){return m})),a.d(e,"q",(function(){return h})),a.d(e,"o",(function(){return f})),a.d(e,"r",(function(){return y})),a.d(e,"n",(function(){return b})),a.d(e,"c",(function(){return g})),a.d(e,"a",(function(){return v})),a.d(e,"d",(function(){return j})),a.d(e,"b",(function(){return D}));var n=a("b775"),i={sys_dict_page:"/admin/sys_dict_page",sys_dict_layer_top:"/admin/sys_dict_layer_top",sys_dict_add:"/admin/sys_dict_add",sys_dict_update:"/admin/sys_dict_update",sys_dict_delete:"/admin/sys_dict_remove",sys_permission_page:"/admin/sys_permission_page",sys_permission_add:"/admin/sys_permission_add",sys_permission_update:"/admin/sys_permission_update",sys_permission_delete:"/admin/sys_permission_remove",sys_role_page:"/admin/sys_role_page",sys_role_add:"/admin/sys_role_add",sys_role_update:"/admin/sys_role_update",sys_role_delete:"/admin/sys_role_delete",sys_permission_all:"/admin/sys_permission_all",sys_permission_layer_top:"/admin/sys_permission_layer_top",sys_user_page:"/admin/sys_user_page",sys_user_add:"/admin/sys_user_add",sys_user_update:"/admin/sys_user_update",sys_user_remove:"/admin/sys_user_remove",sys_role_layer_top:"/admin/sys_role_layer_top",user:"/admin/user",role:"/admin/role",service:"/service",permission:"/permission",permissionNoPager:"/permission/no-pager",orgTree:"/org/tree"};function s(t){return Object(n["b"])({url:i.sys_permission_page,method:"post",data:t})}function o(t){return Object(n["b"])({url:i.sys_permission_add,method:"post",data:t})}function d(t){return Object(n["b"])({url:i.sys_permission_update,method:"post",data:t})}function r(t){return Object(n["b"])({url:i.sys_permission_delete,method:"post",data:t})}function l(t){return Object(n["b"])({url:i.sys_permission_layer_top,method:"post",data:t})}function u(t){return Object(n["b"])({url:i.sys_role_page,method:"post",data:t})}function c(t){return Object(n["b"])({url:i.sys_role_add,method:"post",data:t})}function _(t){return Object(n["b"])({url:i.sys_role_update,method:"post",data:t})}function p(t){return Object(n["b"])({url:i.sys_role_delete,method:"post",data:t})}function m(t){return Object(n["b"])({url:i.sys_user_page,method:"post",data:t})}function h(t){return Object(n["b"])({url:i.sys_user_remove,method:"post",data:t})}function f(t){return Object(n["b"])({url:i.sys_user_add,method:"post",data:t})}function y(t){return Object(n["b"])({url:i.sys_user_update,method:"post",data:t})}function b(t){return Object(n["b"])({url:i.sys_role_layer_top,method:"post",data:t})}function g(t){return Object(n["b"])({url:i.sys_dict_page,method:"post",data:t})}function v(t){return Object(n["b"])({url:i.sys_dict_add,method:"post",data:t})}function j(t){return Object(n["b"])({url:i.sys_dict_update,method:"post",data:t})}function D(t){return Object(n["b"])({url:i.sys_dict_delete,method:"post",data:t})}},"80d6":function(t,e,a){"use strict";a.r(e);var n=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",{staticClass:"dataBody"},[a("a-form",{attrs:{layout:"inline"}},[a("a-form-item",[a("a-input",{attrs:{placeholder:"请输入名称",allowClear:!0},model:{value:t.queryData.name,callback:function(e){t.$set(t.queryData,"name",e)},expression:"queryData.name"}})],1),a("a-form-item",{attrs:{"wrapper-col":{span:12,offset:5}}},[a("a-button",{attrs:{type:"primary","html-type":"submit"},on:{click:t.fetch_no_page}},[t._v(" 查询 ")])],1)],1),a("div",{staticClass:"operate"},[a("a-button",{staticStyle:{width:"100%"},attrs:{type:"dashed",icon:"plus"},on:{click:t.addData}},[t._v("添加")])],1),a("a-table",{attrs:{columns:t.columns,rowKey:function(t){return t.id},dataSource:t.data,pagination:t.pagination,loading:t.loading,indentSize:15,childrenColumnName:"childs"},on:{change:t.handleTableChange},scopedSlots:t._u([{key:"action",fn:function(e){return[a("div",{staticStyle:{width:"110px"}},[a("a",{staticStyle:{"padding-right":"5px"},on:{click:function(a){return t.handleAddChild({parent_id:e.id})}}},[t._v("添加下级")]),a("a-dropdown",[a("a",{staticClass:"ant-dropdown-link"},[t._v(" 更多 "),a("a-icon",{attrs:{type:"down"}})],1),a("a-menu",{attrs:{slot:"overlay"},slot:"overlay"},[a("a-menu-item",[a("a",{staticStyle:{color:"#1890ff"},on:{click:function(a){return t.handleEdit(e)}}},[t._v("编辑")])]),a("a-menu-item",[a("a",{staticStyle:{color:"#f5222d"},on:{click:function(a){return t.handleDelete(e)}}},[t._v("删除")])])],1)],1)],1)]}}])}),t.visible?a("a-modal",{attrs:{title:"add"===t.dialogMode?"添加":"编辑",cancelText:"取消",okText:"确定",width:500,maskClosable:!1},on:{ok:t.handleAddData},model:{value:t.visible,callback:function(e){t.visible=e},expression:"visible"}},[a("a-form",t._b({attrs:{labelAlign:"right"}},"a-form",{labelCol:{sm:{span:4}},wrapperCol:{sm:{span:20}}},!1),[a("a-form-item",{attrs:{label:"名称"}},[a("a-input",{attrs:{placeholder:"名称"},model:{value:t.dialogData.name,callback:function(e){t.$set(t.dialogData,"name",e)},expression:"dialogData.name"}})],1),a("a-form-item",{attrs:{label:"权限集"}},[t.loading_all_res?a("a-spin"):t._e(),a("a-tree",{attrs:{disabled:t.loading_all_res,"selected-keys":t.dialogData.resource_ids,"replace-fields":{children:"childs",title:"name",key:"id"},"auto-expand-parent":!0,"tree-data":t.all_res,checkable:!0},model:{value:t.dialogData.resource_ids,callback:function(e){t.$set(t.dialogData,"resource_ids",e)},expression:"dialogData.resource_ids"}})],1)],1)],1):t._e()],1)},i=[],s=(a("d3b7"),a("5530")),o=a("0fea"),d=a("25a9"),r=[{title:"id",dataIndex:"id"},{title:"名称",dataIndex:"name"},{title:"创建时间",dataIndex:"create_date"},{title:"操作",scopedSlots:{customRender:"action"}}],l={mounted:function(){this.fetch(),this.getAllRes()},data:function(){return{data:[],pagination:{},loading:!1,columns:r,queryData:{id:null,name:null,page_no:1,page_size:5},dialogData:{id:null,value:null},visible:!1,dialogMode:"add",all_res:[],loading_all_res:!1}},methods:{handleTableChange:function(t,e,a){var n=Object(s["a"])({},this.pagination);n.current=t.current,n.pageSize=5,this.pagination=n,this.queryData.page_no=t.current,this.fetch()},fetch_no_page:function(){this.pagination.current=1,this.queryData.page_no=1,this.queryData.page_size=5,this.fetch()},fetch:function(){var t=this;this.loading=!0;var e=Object.assign({},this.queryData);null!=e.time_start&&(e.time_start=e.time_start.format("YYYY-MM-DDThh:mm:ss")),null!=e.time_end&&(e.time_end=e.time_end.format("YYYY-MM-DDThh:mm:ss")),Object(o["k"])(e).then((function(e){var a=Object(s["a"])({},t.pagination);t.loading=!1,t.data=e.data.records,a.total=e.data.total,a.pageSize=e.data.page_size,t.pagination=a}))},addData:function(){this.handleDialogCancel(),this.visible=!0,this.dialogMode="add"},handleAddData:function(){var t=this;"add"===this.dialogMode?Object(o["i"])(this.dialogData).then((function(e){Object(d["a"])(t,e),t.visible=!1,t.fetch()})):"edit"===this.dialogMode&&Object(o["l"])(this.dialogData).then((function(e){t.visible=!1,t.fetch()}))},handleAddChild:function(t){this.visible=!0,this.dialogMode="add",this.dialogData=Object.assign({},t)},handleEdit:function(t){this.visible=!0,this.dialogMode="edit",this.dialogData=Object.assign({},t)},handleDelete:function(t){var e=this;this.$confirm({title:"你确定要删除?",content:"你确定要删除！",onOk:function(){Object(o["j"])(t).then((function(t){Object(d["a"])(e,t),e.visible=!1,e.fetch()}))},onCancel:function(){},class:"test"})},handleDialogCancel:function(){this.dialogData={id:null,remark:null,value:null}},getAllRes:function(){var t=this;this.loading_all_res=!0,Object(o["m"])({}).then((function(e){t.all_res=e.data,t.loading_all_res=!1})).catch((function(e){t.loading_all_res=!1}))}}},u=l,c=(a("bc78"),a("2877")),_=Object(c["a"])(u,n,i,!1,null,null,null);e["default"]=_.exports},bc78:function(t,e,a){"use strict";var n=a("08ed"),i=a.n(n);i.a}}]);