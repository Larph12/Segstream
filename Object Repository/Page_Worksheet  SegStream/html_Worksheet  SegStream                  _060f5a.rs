<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Worksheet  SegStream                  _060f5a</name>
   <tag></tag>
   <elementGuidId>33576f96-fad2-469f-86c8-bfc5f4ddaaa3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    Worksheet | SegStream
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    

    
    
    
    
    
    
    

    var is_all_items_loaded = true;
    var locked_items = [];
    var filter_data = {};


    $(document).ready(function () {

        $('#id_filter_purpose_description').val(filter_data.purpose_description)
        $('#id_new_filter_location').val(filter_data.location)
        $('#id_new_filter_building').val(filter_data.building)
        $('#id_new_filter_asset_class').val(filter_data.asset_class)
        $('#id_new_filter_recovery_period').val(filter_data.recovery_period)
        $('#id_new_filter_quantity').val(filter_data.quantity)
        $('#id_new_filter_division').val(filter_data.division)
        $('#id_new_filter_type').val(filter_data.record_type)
        $('#id_new_filter_takeoffcost').val(filter_data.takeoff_cost_association)
        $('#id_new_filter_unit').val(filter_data.units)
        $('#id_new_filter_costsource').val(filter_data.cost_source)
        $('#id_new_filter_module').val(filter_data.modules);
        $('#id_new_filter_keyword').val(filter_data.keyword);

        $('#sidebarCollapse').on('click', function () {
            $('#sidebar').toggleClass('active');
            if($('#sidebar').hasClass('active')){
                $(&quot;#content&quot;).removeClass('content-active-block');
                $(&quot;#content&quot;).addClass('content-inactive-block');

            }else{
                $(&quot;#content&quot;).removeClass('content-inactive-block');
                $(&quot;#content&quot;).addClass('content-active-block');
            }
        });
        $('input[type=radio][name=view_mode]').change(function() {
            $(&quot;#view_mode_form&quot;).submit();
        });
        navActionFixed();

        format_numbers();

        $(document).on('click', '[item-asb]', function(e){
            e.preventDefault();
            if (!isAssemblyTitlesLock){
                item_asb = $(this).attr('item-asb');
                $(&quot;.&quot; + item_asb).toggle()
            }
        });

        $(document).on('click', '#load_more_view_by_input', function(e){
            var start = $('#load_more_start_input').val();
            is_all_items_loaded = false;
            lazyLoadView(start, 'input_order', 'None');
        });

        
            
        

    });
    $(window).on('load', function() {
        window.localStorage.setItem('view', 'input_order');
    });


    $(document).ready(function(){
        if($('#id_view_by_input_order_tbody tr:visible').length > 0 &amp;&amp; JSON.parse(localStorage.getItem(&quot;show_modification_message&quot;))){
            localStorage.setItem(&quot;show_modification_message&quot;, false);
            $(&quot;#show-modification-info&quot;).hide();
        }
    })
    function lazyLoadView(start, view, building_id){
        if (view == 'modify_order'){
            var url = '/project/view/3338/modify-order/';
        }
        else if(view == 'reverse_modify_order'){
            var url = '/project/view/3338/reverse-modify-order/';
        }
        else if(view == 'input_order'){
            var url = '/project/view/3338/input-order/';
        }else if(view == 'default_order'){
            var url = '/project/view/3338/default-order/';
        }else
        {
            var url = '/project/view/3338/reverse-input-order/';
        }

        $.ajax({
            url: url,
            type: 'POST',
            data: {
                'start': start,
                'building_id': building_id,
                'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;,
                'previous_locked_items': JSON.stringify(locked_items)
            },
            success: function(response) {
                $('#id_view_by_input_order_tbody').append(response.data);
                if($('#id_view_by_input_order_tbody tr:visible').length > 0 &amp;&amp; JSON.parse(localStorage.getItem(&quot;show_modification_message&quot;))){
                    localStorage.setItem(&quot;show_modification_message&quot;, false);
                    $(&quot;#show-modification-info&quot;).hide();
                }
                locked_items = response.locked_items;
                unlocked_items = $(response.unlocked_items);
                unlocked_items.filter('tr').each(function(){
                    $('#'+$(this).attr('id')).replaceWith($(this));
                })
                format_numbers();
                moveNextQuantityEventBind();
                add_border();
                if(toggleCost == true){
                    keep_toggle_cost(isToggleCost);
                }

                if (response.more){
                    // lazyLoadView(response.start, view, building_id);
                    $('#load_more_start_input').val(response.start)
                    $('#load_more_view_by_input').show()
                }else{
                    $('#load_more_view_by_input').hide()
                    is_all_items_loaded = true;
                    updateSummaryCost();
                }
            }
        });
    }

    function refresh() {
        if(new Date().getTime() - INACTIVE_TIME >= 30000000){
            takeoffsheet_lock_expire('3338');
        }
        else if(new Date().getTime() - INACTIVE_TIME &lt;= 10000){
            takeoffsheet_lock_renew('3338');
            setTimeout(refresh, 10000);
        }
        else{
            setTimeout(refresh, 10000);
        }
    }

    setTimeout(refresh, 5000);



    
        
            var read_only_view_enabled = false;
        

        
        var enable_new_ui = false;
        

      window.intercomSettings = {
        app_id: &quot;xtl1gbtq&quot;,
        subdomain: &quot;DEV&quot;,
        
            user_id: &quot;32_DEV&quot;,
            user_hash: &quot;5c77bcfa4bc4cba3cfd1f66c245b1464ccf196d05f59c7a6a4921087c2f566f1&quot;,
            name: &quot;Automation TestUser&quot;,
            email: &quot;info@segstream.com&quot;,
            created_at: &quot;1623259345000&quot;,
            organization: &quot;New AGH&quot;,
            username: &quot;TestAutomationUser&quot;
        
      };
    
    (function(){var w=window;var ic=w.Intercom;if(typeof ic===&quot;function&quot;){ic('reattach_activator');ic('update',w.intercomSettings);}else{var d=document;var i=function(){i.c(arguments);};i.q=[];i.c=function(args){i.q.push(args);};w.Intercom=i;var l=function(){var s=d.createElement('script');s.type='text/javascript';s.async=true;s.src='https://widget.intercom.io/widget/xtl1gbtq';var x=d.getElementsByTagName('script')[0];x.parentNode.insertBefore(s,x);};if(w.attachEvent){w.attachEvent('onload',l);}else{w.addEventListener('load',l,false);}}})();

    
        var storageVersion = '20190607';

        var storage = window.localStorage;

        savedStorageVersion = storage.getItem('storageVersion');
        if (storageVersion != savedStorageVersion){
            storage.setItem('showModal', '');
            storage.setItem('showAddItemModal', '');
            storage.setItem('search_assembly_group_no', '');
            storage.setItem('constructionNumber', '');
            storage.setItem('assemblyNumber', '');
            storage.setItem('toggleCost', '');
            storage.setItem('view', '');
            storage.setItem('NewItemId', '');
            storage.setItem('Xoffset', '');
            storage.setItem('Yoffset', '');
            storage.setItem('ParentId', '');
            storage.setItem('NewCustomAssembly', '');
            storage.setItem('SameItemAgain', '');
            storage.setItem('inputs', '');
            storage.setItem('openProjects', '');
            storage.setItem('openEntities', '');
            storage.setItem('input_order', '');

            storage.setItem('storageVersion', storageVersion);
        }
    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}


    .nav-tabs > li.active > a,
    .nav-tabs > li.active > a:hover,
    .nav-tabs > li.active > a:focus{
        color: #fff;
        background-color: #0747a6;
    }
    tbody{
        cursor: pointer;
    }
    .table-hover tbody tr:hover td, .table-hover tbody tr:hover th {
        background-color: #0747a6;
        color: #fff;
    }

    .has-error .help-block, .has-error .control-label, .has-error .radio, .has-error .checkbox, .has-error .radio-inline, .has-error .checkbox-inline, .has-error.radio label, .has-error.checkbox label, .has-error.radio-inline label, .has-error.checkbox-inline label, .has-error .form-control-feedback, .has-error label{
        color: #a94442;
    }

    input.apple-switch {
  position: relative;
  -webkit-appearance: none;
  outline: none;
  width: 35px !important;
  height: 20px !important;
  background-color: #fff;
  border: 1px solid #D9DADC;
  border-radius: 50px;
  box-shadow: inset -20px 0 0 0 #fff;
}

input.apple-switch:after {
    content: &quot;&quot;;
    position: absolute;
    top: 1px;
    left: 1px;
    background: transparent;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    box-shadow: 2px 4px 6px rgba(0,0,0,0.2);
}

input.apple-switch:checked {
    box-shadow: inset 16px 0 0 0 #a7c7df;
    border-color: #a7c7df;
}

input.apple-switch:checked:after {
  left: 15px;
  box-shadow: -2px 4px 3px rgba(0,0,0,0.05);
}
    .analysis_window_tab:hover{
        cursor: pointer;
    }

    .has-error .form-control, .has-error .form-control:focus{
        border: 1px solid #a94442;
    }
    .avatar_img {
        overflow: hidden;
        text-align: center;
        position: relative;
    }
    .avatar_img > img {
        position: absolute;
        height: 100%;
        -webkit-transform: translateX(-50%);
        -moz-transform: translateX(-50%);
        transform: translateX(-50%);
    }
    #id_item{
        background-color: #fff!important;
    }
    input[type=checkbox] {width:20px; height:17px;}
    .tooltip-inner {
        max-width: 550px;
        font-size: 13px;
        text-align: justify;
    }
    .squeeze_warning_button_border{
        border:solid;
        border-color:red;
    }

    
    tr.row_5Y {
        background-color: #a0dd84 !important;
    }
    .BOX_5Y {
        background-color: #a0dd84 !important;
        border-color: #a0dd84 !important;
    }
    .AW_5Y {
        background-color: #a0dd84 !important;
        border: 2px solid #a0dd84 !important;
        font-weight: bold;
    }
    .AW_BORDER_5Y {
        border: 2px solid #a0dd84 !important;
        font-weight: bold;
    }

    tr.lock{
        display: none;
    }

tr._5Yassemb_child{
    background: #a0dd84;
}

.completed-algorithm{
    color: blue;
}

/*##################INPUT BACKGROUND COLOR*###########*/

.color_5Y{
    background: {{} cl.color_code }!important;
}
    
    tr.row_15Y {
        background-color: #f9f59a !important;
    }
    .BOX_15Y {
        background-color: #f9f59a !important;
        border-color: #f9f59a !important;
    }
    .AW_15Y {
        background-color: #f9f59a !important;
        border: 2px solid #f9f59a !important;
        font-weight: bold;
    }
    .AW_BORDER_15Y {
        border: 2px solid #f9f59a !important;
        font-weight: bold;
    }

    tr.lock{
        display: none;
    }

tr._15Yassemb_child{
    background: #f9f59a;
}

.completed-algorithm{
    color: blue;
}

/*##################INPUT BACKGROUND COLOR*###########*/

.color_15Y{
    background: {{} cl.color_code }!important;
}
    
    tr.row_39Y {
        background-color: #F9AA9A !important;
    }
    .BOX_39Y {
        background-color: #F9AA9A !important;
        border-color: #F9AA9A !important;
    }
    .AW_39Y {
        background-color: #F9AA9A !important;
        border: 2px solid #F9AA9A !important;
        font-weight: bold;
    }
    .AW_BORDER_39Y {
        border: 2px solid #F9AA9A !important;
        font-weight: bold;
    }

    tr.lock{
        display: none;
    }

tr._39Yassemb_child{
    background: #F9AA9A;
}

.completed-algorithm{
    color: blue;
}

/*##################INPUT BACKGROUND COLOR*###########*/

.color_39Y{
    background: {{} cl.color_code }!important;
}
    
    tr.row_GDS_sc {
        background-color:  !important;
    }
    .BOX_GDS_sc {
        background-color:  !important;
        border-color:  !important;
    }
    .AW_GDS_sc {
        background-color:  !important;
        border: 2px solid  !important;
        font-weight: bold;
    }
    .AW_BORDER_GDS_sc {
        border: 2px solid  !important;
        font-weight: bold;
    }

    tr.lock{
        display: none;
    }

tr._GDS_scassemb_child{
    background: ;
}

.completed-algorithm{
    color: blue;
}

/*##################INPUT BACKGROUND COLOR*###########*/

.color_GDS_sc{
    background: {{} cl.color_code }!important;
}
    

    tr._GDS_scassemb_child{
    background: #656560;
}

    tr.child_GDS_sc {
        background-color: #979795 !important;
    }

    tr.row_GDS_sc {
        background-color: #979795 !important;
    }
    .AW_GDS_sc {
        background-color: #979795 !important;
        border: 2px solid #979795 !important;
        font-weight: bold;
    }
    .BOX_GDS_sc {
        background-color: #979795 !important;
        border-color: #979795 !important;
    }
    .AW_BORDER_GDS_sc {
        border: 2px solid #979795 !important;
        font-weight: bold;
    }
    .buildings, .multiform-remove {
        /* border: 1px solid gray; */
        /* margin-right: 45px; */
        margin: 0;
        position: relative;
        top: 50%;
        -ms-transform: translateY(-50%);
        transform: translateY(-50%);
    }
    .multiform-remove{
        float:right;
        margin-right: 20px
    }
    div[id$='item_container'] {
        position: relative;
        height: 200px;
        border: solid 1px gray;
        border-radius: 20px;
        margin-top: 20px
    }
    .location_in_use_links div:hover, .asset_in_use_links div:hover{
        text-decoration: underline;
    }
#notification-menu{
  top: 60px;
  right: 0px;
  left: unset;
  width: 460px;
  box-shadow: 0px 5px 7px -1px #c1c1c1;
  padding-bottom: 0px;
  padding: 0px;
}
#notification-menu:before{
  content: &quot;&quot;;
  position: absolute;
  top: -20px;
  right: 12px;
  border:10px solid #343A40;
  border-color: transparent transparent #343A40 transparent;
}
.head{
  padding:5px 15px;
  border-radius: 3px 3px 0px 0px;
}
.footer{
  border-radius: 0px 0px 3px 3px;
}
.head a, .footer a{
  background: gray;
}
.notification-box{
  padding: 10px 10px;
  border-bottom: 1px solid #ddd;
  background: white;
}
.bg-gray{
  background-color: #e7e6f1;
}

.float-right{
    float: right;
    background: gray;
}

.notification-box > div > strong {
    font-weight: bold;
    font-size: 13px;
}

.notification-box > p > strong {
    font-weight: bold;
    font-size: 14px;
}

.notification-box > p{
    margin-top: 0;
    margin-bottom: 5px;
}

.notification-box a{
    background: none;
    padding:0px !important;
    color: blue;
}

.notification-box a:hover {
    background: none;
    padding: 0px !important;
    text-decoration: underline;
}

.text-warning {
    color: #727272;
}

@media (max-width: 640px) {
    .dropdown-menu{
      top: 50px;
      left: -16px;
      width: 290px;
    }
    .nav{
      display: block;
    }
    .nav .nav-item,.nav .nav-item a{
      padding-left: 0px;
    }
    .message{
      font-size: 13px;
    }
}

.profile-pic {
    position: relative;
    font-size: 0.9em;
    color: grey;
    cursor: pointer;
    margin-right: 10px;
}

span.fa-circle {
    position: absolute;
    font-size: 2.3em;
    top: -20px;
    color: #c00;
    right: -3px;
}

span.num {
    position: absolute;
    font-size: 1.2em;
    top: -17px;
    color: #fff;
    right: 2px;
}

.dropdown-menu>li>a{
    display: block;
    padding: 2px 20px;
    clear: both;
    font-weight: 400;
    line-height: 2.6;
    color: #333;
    white-space: nowrap;
    background: #fff;
}

.dropdown-menu>li{
    height: 35px;
}

.dropdown-menu>li>a:hover{
    background-color: #c00;
    color: #fff;
}

.notification_popup {
    /* filter: blur(0px); */
    opacity: 95%;
    transform: scale(0.8);
  /*animation: wiggle 2.5s infinite;*/
}


@keyframes wiggle {
    0% { transform: scale(0.8) translateX(0); }
   25% { transform: scale(0.8) translateX(10px); }
   50% { transform: scale(0.8) translateX(0px); }
   75% { transform: scale(0.8) translateX(-10px); }
  100% { transform: scale(0.8) translateX(0px); }
}

.notification_popup:hover {
  animation: none;
  transform: scale(0.9);
}

.notification_popup .myBar, .notification_popup .myProgress{
    display: block !important;
}

.myBar {
    width: 0%;
    /* height: 30px; */
    height: 5px;
    background-color: white;
}
.myProgress{
    width: 100%;
    height: 5px;
    background: #486fa5;
    padding: 0;
    margin: 0;
}

.notification-container{
    max-height: 365px;
}

.notification_popup .notification-container{
    max-height: 250px;
}

.close-notification-popup {
    background: white;
    /* height: 30px; */
    /* width: 47px; */
    border-radius: 10px;
    /* padding: 5px; */
    width: 20px;
    height: 20px;
    padding-left: 7px;
    cursor: pointer;
}

.notification_popup .close-notification-popup{
    display: block;
}

#dropdown_menu_container:not(.notification_popup) .close-notification-popup{
    display:none;
}

.notification_popup .go-back-to-home{
    display: none;
}



    
        
            
                Toggle navigation
                
                
                
            
            
                
            
        
        
        
            
                 Add Takeoff or Costs
                 Add Custom Assembly
                
                     Apply Squeeze
                     Reset Squeeze
                     Edit Property
                
                 Export 
                 Controls
                 Help
            
        

        
            
                
                    
                        
                            
                            
                                0
                            
                         Automation  
                    

                    

                    
                
                
                
            
        
        
    


    
                          
                        
                      
                        
                          
                             Notification
                            x
                          
                      
                      
                          
                      
                      View All
                    

                        
                        
                        
                            
                        
                        Automation
                    
                    New AGH
                    
                    
                    
                        
                        
                        Notification 0
                        
                        Manage Custom Assembly
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        Change Password
                        Logout
                    
                    
                        
                        
                          
                             Management
                          
                      
                        Manage Purpose Description
                        
                        Manage Location
                        
                        Manage Custom Unit
                        
                        Manage Cost Source
                        
                        Manage Depreciation Methods
                        
                        Manage Algorithm
                        Manage Master Algorithm
                    

    
        
    
    
        
        
          

        
        
          
            
              We are Building your Property.
            
            So please grab a coffee and let us do the heavy lifting.
            
                
            
          
        
      
    
    
        
        
          ×
          Your report is being generated. Be patient!
        

        
          ×
          We're working on your Modification, we will begin displaying your Worksheet as soon as we're done processing it...
        
        
    
    
        
            
            
        
    

    
    
        

    #location-in-use:hover, #recovery-period-in-use:hover, #algorithm-in-use:hover, .activate{
        color: #0747A6;
        background: #fff;
    }


    
        Property
        DB
    
    
        

            
                 Projects 
                
                    
                         Recent Properties 
                        
                            
                                No recent Properties opened
                            
                        
                    

                    
                         Properties in Entity 
                        
                            
                        
                    
                    
                         Back to Dashboard
                    
                
            
        

        

        
            
                 Costs Setup 
                
                    Indirect Costs
                    Direct Costs
                
            

            
                
                    
                    
                        Rapid Takeoff Entry Mode
                    
                    
                        
                    
                
            
            
            
                 Bulk Select 
                
                    
                         Select all
                    
                    
                         Select inverse
                    
                    
                    Select By Location
                    Select By Recovery Period
                    Select By Asset Class
                    Select By Construction Division
                    Find and Replace
                    Advanced
                
                 Bulk Modify 
                
                    Delete Selected
                    Delete Zero Quantities
                    
                    Modify Location of Selected
                    Modify Recovery Period of Selected
                    Modify Asset Class of Selected
                    Zero All Quantities
                    Multiply Locations
                    Map Locations
                    Map Takeoff Costs
                    Map Recovery Period
                    Map Purpose Descriptions to Asset Classes
                    Map Asset Classes to Recovery Period
                    Clone Selected
                    Advanced
                

                
                
            
            
            
                
                    
                    View by
                        
                        
                         Input Order
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                        
                    
                    
                
                
                    
                        
                        
                        

                        
                            Read-only view
                        

                        
                            
                                View by Input Order
                            
                            In Order
                        
                        
                            Reverse
                        
                        
                            View by Last Modified 
                        

                        
                            View by Item Type 
                        
                        
                            
                                View by Item
                            
                            Item Number
                        
                        
                            Alphabetical
                        
                        
                            View by Recovery Period
                        
                        
                            View by Asset Class
                        
                        
                        
                            View by Building System
                        
                        
                            View by Division
                        
                        
                            View by Location
                        
                        
                            View by Takeoff Cost
                        
                        
                            View by Uniformat Group
                        
                        
                            
                                View by Item Total Cost
                            
                            Low to High
                        
                        
                            High to Low
                        
                    
                
            
                

                    
                    
                        
                            
                                Locations in Use
                            
                            
                                Asset Classes in Use
                            
                        
                        
                            
                                
                                    
                                    
                                    
                                    
                                    
                                
                            
                            
                            
                                
                                    
                                        No Asset Class
                                    
                                
                            
                            
                    
                    
                
        
    
    
    



    $(document).on(&quot;change&quot;, &quot;#selected_algorithm_tag&quot;, function(){
        window.location.href = &quot;/algorithm/?id=32_DEV&amp;tag=&quot;+$(this).val();
    });
    $(document).ready(function(){
        $(document).on('change keyup', '.building_types', function(){
            var view = $('input[name=&quot;view_mode&quot;]:checked').val();
            var order = $('input[name=&quot;sort_order&quot;]:checked').val();
            filter_ajax_call(view, order);
        });

        $(document).on('change', '.view_by_input', function(){
            var view = $(this).val();
            $(&quot;#id_select_view&quot;).text($(this).data(&quot;label&quot;))
            var order = $('input[name=&quot;sort_order&quot;]:checked').val();
            filter_ajax_call(view, order);
        });

        $(document).on('click', '#reset_button', function(){
            $('input[name=&quot;sort_order&quot;][value=&quot;oldest&quot;]').prop('checked', true);
            $('input[name=&quot;view_mode&quot;][value=&quot;projects&quot;]').prop('checked', true);
            $('#organization_user').val('')
            $('#building_select').val('');
            $('#property_select').val('');
            $('#zip_code').val('');
            $('#search').val('');
            $('#access_type').val('all');

            var order = $('input[name=&quot;sort_order&quot;]:checked').val();
            var view = $('input[name=&quot;view_mode&quot;]:checked').val();
            $('#label_list_dashboard').html('Project');
            filter_ajax_call(view, order, state=true);
        });

        $(document).on('change', '.sort_order', function(){
            var order = $(this).val();
            var view = $('input[name=&quot;view_mode&quot;]:checked').val();
            filter_ajax_call(view, order);
        });

        $(&quot;.location_in_use_links&quot;).click(function(){
            $(&quot;#location_in_use_clicked&quot;).val($(this).data(&quot;lid&quot;));
            $(&quot;#building_in_use_clicked&quot;).val($(this).data(&quot;buildid&quot;));
            $(&quot;#location_view_mode_form&quot;).submit();
        })

        $(&quot;.asset_in_use_links&quot;).click(function(){
            $(&quot;#asset_in_use_clicked&quot;).val($(this).data(&quot;aid&quot;));
            $(&quot;#asset_view_mode_form&quot;).submit();
        });

        $(&quot;#id_assembly_title_lock_control&quot;).click(function(){
            isAssemblyTitlesLock = $(this).prop(&quot;checked&quot;)
        })

        $(document).on('click', '#location-in-use', function(){

            $('#id_locations_detail').show();
            $('#id_locations_detail').siblings('ul').hide();

            $(this).addClass('activate');
            $(this).siblings().removeClass('activate')
        });

        $(document).on('click', '#algorithm-in-use', function(){
            $('#id_algorithm_detail').show();
            $('#id_algorithm_detail').siblings('ul').hide();

            $(this).addClass('activate');
            $(this).siblings().removeClass('activate')
        });

        $(document).on('click', '#recovery-period-in-use', function(){
            $('#id_recovery_period_detail').show();
            $('#id_recovery_period_detail').siblings('ul').hide();

            $(this).addClass('activate');
            $(this).siblings().removeClass('activate')
        });
    });

    function filter_ajax_call(view, order, state=false){
        var organization_user = $('#organization_user').val();
        var build_value = $('#building_select').val();
        var prop_value = $('#property_select').val();
        var zip_code = $('#zip_code').val();
        var query = $('#search').val();
        var access = $('#access_type').val();
        $('#add_property').hide();
        $('#label_list_dashboard').parent('h4').css('margin-left', '10px');
        $('#label_list_dashboard').html('Project');
        $('#id_processing').show();

        var reset_state = ''

        if (state){
            reset_state = 'reset_state'
        }

        if(view == 'property'){
            $('#add_entity').show();
            $('#add_property').show();
            $('#label_list_dashboard').parent('h4').css('margin-left', '0px');
            $('#label_list_dashboard').html('Property');
        } else if(view == 'entity'){
            $('#add_entity').show();
            $('#label_list_dashboard').parent('h4').css('margin-left', '14px');
            $('#label_list_dashboard').html('Entity');
        }

        $.ajax({
            type: &quot;POST&quot;,
            url: &quot;/&quot;,
            data: {
                'build_id': build_value,
                'organization_user': organization_user,
                'prop_type': prop_value,
                'zip_code': zip_code,
                'q': query,
                'view_mode': view,
                'access_type': access,
                'sort_order': order,
                'reset_state': reset_state,
                &quot;csrfmiddlewaretoken&quot;: &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;
            },
            cache: false,
            success: function(response){
                $('#id_processing').hide();
                $('#project_list_div').html(response.data);

            }
        });

    }


        
        
        
            
    
        
            
                
                Show/Hide Children
                Toggle Record Info
                Toggle Cost
                
                Export Normalized Quantity
                Import RS Means Items
            
        
    


            
                    
                        Description
                        Quantity
                        Unit
                        Material
                        Labor
                        Active Total Cost
                        Initial Total Cost
                        Modified Total Cost
                        Squeeze Total Cost
                    
                
                
                    
                        Description
                        Quantity
                        Unit
                        Material
                        Labor
                        Active Total Cost
                        Initial Total Cost
                        Modified Total Cost
                        Squeeze Total Cost
                    
                
                
                    
                        
                            
                                No items in this Worksheet
                            
                        
                    
                
            
            
        
        
            


    Filter 



     Information
    
        Property Name : April 21
        Tie-To Number : 
        Placed-In-Service Date : 
        
    


    Property Name : April 21
    Entity Name : Test_1
    Project Name : April Test Project
    Tie-To Number : 
    Placed-In-Service Date : 
    Job Number : 
    
    City, State, Zipccode : Ackworth, IA, 50001
    Total Building SF : 0
    Depreciation Method : Commercial MACRS (GDS)
    Project Type : Purchase Price Allocation (No Given Costs)
    Number of Assemblies : 0
    Number of Individual Items: 0
    Total Number of Lines : 0
    

 Analysis Window


    
        
        
        
        
        
            
                
                    5 Years :
                
                0.00
                (0.00%)
            
        
        
        
            
                
                    15 Years :
                
                0.00
                (0.00%)
            
        
        
        
            
                
                    39 Years :
                
                0.00
                (100.00%)
            
        
        
        
        
    

    
        
            Initial Total Cost
        
        0.00
    
    
    
        
            Active Total Cost
        
        0.00
    
    
    


    
        
            
            WARNING:
                You have made changes to the Worksheet after a squeeze.
                
                Please reset the squeeze and then squeeze the sheet again when you have finished making changes by using the Reset Squeeze button and then use the Apply Squeeze button at the top of the screen to perform ALL Squeezes you want implemented again.
                
        
    








    $(&quot;#id_info_head&quot;).click(function(){
        var selectedEffect = &quot;drop&quot;;
        $(&quot;#id_info_detail_short&quot;).toggle();
        $(&quot;#id_info_detail&quot;).toggle( selectedEffect, 500, callback );
    });

    $(&quot;#id_locations_used&quot;).click(function(){
        var selectedEffect = &quot;drop&quot;;
        $(&quot;#id_locations_detail&quot;).toggle( selectedEffect, 500, callback );
    });

    function callback() {
        setTimeout(function() {
            $( &quot;#effect&quot; ).removeAttr( &quot;style&quot; ).hide().fadeIn();
        },400 );
    };

    $(document).ready(function(){
        var tmp = $('span#id_tie_to_no').html().replace(/[$]/, '');
        $('span#id_tie_to_no').html(tmp)

        $('.asset_year_in_use_links').hover(
            function(){
                $(this).addClass('add-pointer');
                $(this).closest('div').find('.asset_year_name').addClass('text-decorate');
            }, function(){
                $(this).removeClass('add-pointer');
                $(this).closest('div').find('.asset_year_name').removeClass('text-decorate');
            }
        );

        $('.asset_year_in_use_links').on('click', function(){
            $(&quot;#asset_year_in_use_clicked&quot;).val($(this).data(&quot;code&quot;));
            $('#asset_year_view_mode_form').submit();
        })
    });


        
    


    
        Click to return on the top page
    

    
    
        
    
        
            
                ×
                 Apply Squeezing
            
            
                
                
                
                    
                        
                            
                                
                                    Squeeze Type
                                
                                
                                    
                                        Select
                                        
                                            1. Squeeze Entire Worksheet
                                            
                                            2. Squeeze by Takeoff Cost
                                            3. Squeeze by Uniformat/Assembly Group
                                        
                                    
                                
                            
                            
                                
                                    
                                        Total Building Square Footage
                                    
                                    
                                        
                                    
                                
                                
                            
                            
                            
                                
                                    Tie-to Number
                                
                                
                                    
                                        $
                                        
                                    
                                    
                                
                            

                            
                                
                            
                        
                    
                
                
                    Apply Squeeze
                    Cancel
                
            
        
    


    $(&quot;#apply_squeeze_button_action&quot;).on(&quot;click&quot;, function(e){
        e.preventDefault();
        $(&quot;#applySqueeze&quot;).modal(&quot;hide&quot;);
        $(&quot;#id_processing&quot;).show();
        $(&quot;#apply_squeeze_form&quot;).submit();
    })

    $(&quot;#id_total_square_foot&quot;).keypress(function(e) {
        if (e.which != 8 &amp;&amp; e.which != 0 &amp;&amp; (e.which &lt; 48 || e.which > 57)) {
            if (e.which == 46) {
                return true;
            } else {
                return false;
            }
        }
    });

    $('#id_total_square_foot').keyup(function(e) {
        if ($(this).val() == &quot;0.0&quot;) {
            $(this).val() = &quot;&quot;
        } else if($(this).val()){
            var $this = $(this);
            var num = parseFloat($this.val().replace(/,/g, ''));
            $this.val(num.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        }
    });


        
    
        
            
                ×
                Import RS Means CSV
            
            
                
                
                    
                        
                            
                                
                                    
                                        File: 
                                        
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Upload
                    Cancel
                
            
        
    


    

    
    
        
            
                ×
                Delete Items/Assemblies with zero quantities
            
            
                Are you sure you want to delete all  the records with zero quantity?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                Delete Items/Assemblies with zero quantities
            
            
                Are you sure you want to delete all  the records with zero quantity?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                Set Quantity of records to zero
            
            
                
                
                
                
                
                
                    Are you sure you want to make the quantities of selected  records to zero?
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Revert Cost factor to 1
            
            
                Are you sure you want to revert the cost factors (location, equipment and labour) of the  selected records?
                Note: Custom Items are not affected by Cost Factors
            
            
                Revert the cost factors and modify selected Items again
                Revert the cost factors
                Cancel
            
        
    


    
    
        
            
                ×
                Confirm Revert Cost factor to 1
            
            
                
                
                    Are you sure you want to revert the cost factors (location, equipment and labour) of the filtered records?
                    Note: Custom Items are not affected by Cost Factors
                
                
                
                
                    Revert the cost factors
                    Revert the cost factors and modify selected Items again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                 Add New Project
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Project Name
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Save
                    Cancel
                
            
        
    



    $(document).on('click', '.add-project-save-button', function(e){
        e.preventDefault();
        var valid = true;

        if (!$('#id_title').val()){
            valid = false
            $('.not-valid-error').html('This field is required.')

        }

        if(valid){
            $('.add-project-form').submit();
        }
    })


    
    
        
            
                ×
                Update multiple Quantity
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Quantity of  selected records.
                                        
                                        
                                            Update Final Quantity
                                            
                                        
                                        
                                            Update Unit Quantity
                                            
                                        
                                        
                                            
                                                Final Quantity: 
                                            
                                            
                                                
                                                
                                            
                                        
                                        
                                            
                                                Unit Quantity: 
                                            
                                            
                                                
                                                
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    



    $(document).on('change', '#unit_quantity', function(e){
        $('#id_final_quantity').hide();
        $('#id_unit_quantity').show();
    });

    $(document).on('change', '#final_quantity', function(e){
        $('#id_final_quantity').show();
        $('#id_unit_quantity').hide();
    });

    $(document).on('keyup', '.quantity_update', function(){

        if($(this).val().length > 7){
            if ($('#id_unit_quantity').is(':visible')) {
                $('.unit-error').html('Invalid input')
                $('.submit_quantity').prop('disabled', true);
            }else{
                $('.final-error').html('Invalid input')
                $('.submit_quantity').prop('disabled', true);
            }
        }else{
            $('.final-error').html('')
            $('.unit-error').html('')
            $('.submit_quantity').prop('disabled', false);
        }

    });


    
    
        
            
                ×
                Update multiple Quantity
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Quantity of  selected records.
                                        
                                        
                                            Update Final Quantity
                                            
                                        
                                        
                                            Update Unit Quantity
                                            
                                        
                                        
                                            
                                                Final Quantity: 
                                            
                                            
                                                
                                            
                                        
                                        
                                            
                                                Unit Quantity: 
                                            
                                            
                                                
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    



    $(document).on('change', '#unit_quantity_filter', function(e){
        $('#id_final_quantity_filter').hide();
        $('#id_unit_quantity_filter').show();
    });

    $(document).on('change', '#final_quantity_filter', function(e){
        $('#id_final_quantity_filter').show();
        $('#id_unit_quantity_filter').hide();
    });


    
    
        
            
                ×
                Modify Global Pricing Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Global Pricing Factor of  selected records.
                                        
                                        
                                            Global Pricing Factor: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Modify Global Pricing Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Global Pricing Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Revert Costs to Original Database Values
            
            
                Are you sure you want to Revert Costs to Original Database Values of  selected records?
            
            
                Update
                Update &amp; Modify Selected Items Again
                Cancel
            
        
    


    
    
        
            
                ×
                Confirm Revert Costs to Original Database Values
            
            
                
                
                    Are you sure you want to Revert Costs to Original Database Values of filtered records?
                
                
                
                
                    Update
                    Update &amp; Modify Selected Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                 Update multiple Concationation
            
            
                
                    
                        
                            
                                
                                    
                                        Custom Items are not available for Purpose Description Prefixes. If you would like to change the description of any Custom Item, you may do so by double-clicking on the record for each Custom Item and changing the description inside of the dialogue that appears. The rest of your selection will be concatenated as desired.
                                    
                                
                            
                        
                    
                
            
            
                Close
            
        
    


    
    
        
            
                ×
                Update multiple Purpose Description Prefix
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Custom Items are not available for Purpose Description Prefixes. If you would like to change the description of any Custom Item, you may do so by double-clicking on the record for each Custom Item and changing the description inside of the dialogue that appears. The rest of your selection will be concatenated as desired.
                                        
                                        
                                            
                                                Update Purpose Description of  selected records.
                                            
                                            
                                                Purpose Description Prefix: 
                                            
                                            
                                                
                                                    None
                                                
                                                    A/V System
                                                
                                                    A/V System Equipment
                                                
                                                    A/V System Equipment Electric
                                                
                                                    A/V System,
                                                
                                                    Back Porch
                                                
                                                    Backsplash Wall
                                                
                                                    Balcony Exterior Doors,
                                                
                                                    Balcony Floor Finish,
                                                
                                                    Balcony Steel Railing,
                                                
                                                    Balcony Wall Light,
                                                
                                                    Base
                                                
                                                    Base Molding,
                                                
                                                    Basement Staircase,
                                                
                                                    Bldg
                                                
                                                    Bldg Basement
                                                
                                                    Bldg Basement Concrete Floor,
                                                
                                                    Bldg Basement Excavation, 
                                                
                                                    Bldg Basement FDN Wall
                                                
                                                    Bldg CC Column
                                                
                                                    Bldg CC Elevated Floor Structure
                                                
                                                    Bldg CC Footing
                                                
                                                    Bldg CC Footings
                                                
                                                    Bldg CC Slab
                                                
                                                    Bldg CC Slab on Grade
                                                
                                                    Bldg Column Fireproofing,
                                                
                                                    Bldg Concrete Ext Wall
                                                
                                                    Bldg Concrete Staircase,
                                                
                                                    Bldg Elev Beam FDN
                                                
                                                    Bldg Elev Concrete Floor
                                                
                                                    Bldg Elev Floor Structure
                                                
                                                    Bldg Elev Metal Joist Floor
                                                
                                                    Bldg Elev Structural Steel Floor
                                                
                                                    Bldg Elev Walkway
                                                
                                                    Bldg Elev Wood Joist Floor
                                                
                                                    Bldg Elevated Floor Structure
                                                
                                                    Bldg Ext Brick Stucco Wall
                                                
                                                    Bldg Ext Brick Veneer Wall
                                                
                                                    Bldg Ext Brick Wall
                                                
                                                    Bldg Ext CMU EIFS Wall
                                                
                                                    Bldg Ext CMU Stucco Wall
                                                
                                                    Bldg Ext CMU Wall
                                                
                                                    Bldg Ext Column Covering
                                                
                                                    Bldg Ext Concrete Wall Panels
                                                
                                                    Bldg Ext Demountable Canopy
                                                
                                                    Bldg Ext EIFS Wall
                                                
                                                    Bldg Ext Glass Curtain Wall
                                                
                                                    Bldg Ext Lighting
                                                
                                                    Bldg Ext Metal Panel Curtain Wall
                                                
                                                    Bldg Ext Metal Siding Wall
                                                
                                                    Bldg Ext Metal Stud EIFS Wall
                                                
                                                    Bldg Ext Metal Stud Fiber Cement Wall
                                                
                                                    Bldg Ext Metal Stud Stucco Wall
                                                
                                                    Bldg Ext Metal Stud Wall
                                                
                                                    Bldg Ext Stone Veneer Wall
                                                
                                                    Bldg Ext Tilt-Up CC Panel Wall
                                                
                                                    Bldg Ext Vinyl Siding Wall
                                                
                                                    Bldg Ext Wall
                                                
                                                    Bldg Ext Wall Lighting
                                                
                                                    Bldg Ext Wall Lighting Electrical
                                                
                                                    Bldg Ext Wood Siding Wall
                                                
                                                    Bldg Ext Wood Stud EIFS Wall
                                                
                                                    Bldg Ext Wood Stud Fiber Cement Wall
                                                
                                                    Bldg Ext Wood Stud Stucco Wall
                                                
                                                    Bldg Ext Wood Stud Wall
                                                
                                                    Bldg FDN
                                                
                                                    Bldg Masonry Ext Wall
                                                
                                                    Bldg Metal Ext Wall
                                                
                                                    Bldg PEMB Structure
                                                
                                                    Bldg Porte Cochere Brick Veneer Wall
                                                
                                                    Bldg Porte Cochere Column Covering
                                                
                                                    Bldg Porte Cochere Concrete Footing
                                                
                                                    Bldg Porte Cochere EIFS Wall
                                                
                                                    Bldg Porte Cochere Lighting
                                                
                                                    Bldg Porte Cochere Roof Cover
                                                
                                                    Bldg Porte Cochere Roof Drainage
                                                
                                                    Bldg Porte Cochere Roof Structure
                                                
                                                    Bldg Porte Cochere Steel Beams
                                                
                                                    Bldg Porte Cochere Steel Column
                                                
                                                    Bldg Porte Cochere Stone Veneer Wall
                                                
                                                    Bldg Porte Cochere Wood Beams
                                                
                                                    Bldg Porte Cochere Wood Columns
                                                
                                                    Bldg Porte Cochere Wood Stud Stucco Wall
                                                
                                                    Bldg Roof
                                                
                                                    Bldg Roof Cover
                                                
                                                    Bldg Roof Covering
                                                
                                                    Bldg Roof Drainage
                                                
                                                    Bldg Roof Structure
                                                
                                                    Bldg Staircase,
                                                
                                                    Bldg Steel Column
                                                
                                                    Bldg Steel Railing,
                                                
                                                    Bldg Steel Staircase,
                                                
                                                    Bldg Strip Foundation
                                                
                                                    Bldg Structural Steel Beam
                                                
                                                    Bldg Unit Concrete Staircase,
                                                
                                                    Bldg Unit Staircase,
                                                
                                                    Bldg Unit Steel Staircase,
                                                
                                                    Bldg Unit Wood Staircase,
                                                
                                                    Bldg Walkway Concrete Finishing,
                                                
                                                    Bldg Walkway Steel Railing,
                                                
                                                    Bldg Wd Elevated Floor Structure
                                                
                                                    Bldg Wood Column
                                                
                                                    Bldg Wood Columns
                                                
                                                    Bldg Wood Ext Wall
                                                
                                                    Bldg Wood Staircase,
                                                
                                                    Bldg-Mtd Lighting
                                                
                                                    Break Area
                                                
                                                    Break Area Base
                                                
                                                    Break Area Base Cabs w/Ctr
                                                
                                                    Break Area Base Cabs w/Ctr,
                                                
                                                    Break Area Dbl Sink
                                                
                                                    Break Area Dishwasher Electric,
                                                
                                                    Break Area Elec Water Heater
                                                
                                                    Break Area Equipment
                                                
                                                    Break Area Equipment Electrical
                                                
                                                    Break Area Gas Range/Oven
                                                
                                                    Break Area Sink
                                                
                                                    Break Area Wall
                                                
                                                    Break Area Wall Cabinets
                                                
                                                    Break Room Base Cabs w/Ctr
                                                
                                                    Break Room Equipment
                                                
                                                    Break Room Gas Range/Oven
                                                
                                                    Break Room Sink
                                                
                                                    Break Room Wall Cabinets
                                                
                                                    Breakfast Buffet Base
                                                
                                                    Breakfast Buffet Equipment
                                                
                                                    Building Foundation Wall
                                                
                                                    Building Spread Foundation
                                                
                                                    Building-Mounted
                                                
                                                    Building-Mounted Lighting
                                                
                                                    Built-In Desk,
                                                
                                                    Built-In Wardrobe,
                                                
                                                    CCTV Electrical,
                                                
                                                    Ceiling
                                                
                                                    Ceiling Fan Electrical,
                                                
                                                    Ceilings &amp; Partitions
                                                
                                                    Ceramic Tile
                                                
                                                    Chair Rail
                                                
                                                    Checkout Counter,
                                                
                                                    Chimney
                                                
                                                    Closet
                                                
                                                    Closet Door,
                                                
                                                    Cold Storage Ceiling Unit Cooler,
                                                
                                                    Cold Storage Condenser,
                                                
                                                    Cold Storage Drain Piping,
                                                
                                                    Cold Storage Electric,
                                                
                                                    Cold Storage Refrigeration Piping,
                                                
                                                    Cold Storage Remote Compressor,
                                                
                                                    Cold Storage,
                                                
                                                    Common
                                                
                                                    Common Area
                                                
                                                    Comms/Data Equipment
                                                
                                                    Comms/Data Equipment Electric
                                                
                                                    Compressed Air Equipment Electrical,
                                                
                                                    Compressed Air Equipment,
                                                
                                                    Compressed Air Piping,
                                                
                                                    Computer Equipment
                                                
                                                    Computer Equipment Electric
                                                
                                                    Computer Room HVAC
                                                
                                                    Concrete Topping Floor Finish
                                                
                                                    Condiments &amp; Beverage Counter,
                                                
                                                    Convenience Outlet,
                                                
                                                    Crane Equipment Electrical,
                                                
                                                    Crane Equipment,
                                                
                                                    Crane Rail Steel Support,
                                                
                                                    Crown Molding
                                                
                                                    Data Center Access Flooring, 
                                                
                                                    Data Center Electrical, 
                                                
                                                    Data Center Fire Suppression System, 
                                                
                                                    Data Center HVAC Electrical, 
                                                
                                                    Data Center HVAC, 
                                                
                                                    Data/Comms Equipment
                                                
                                                    Decorative
                                                
                                                    Decorative Pendant
                                                
                                                    Decorative Pendant Light
                                                
                                                    Decorative Pendant Lighting
                                                
                                                    Demountable Mezzanine
                                                
                                                    Demountable Partition, 
                                                
                                                    Demountable PTAC HVAC System Electrical, 
                                                
                                                    Demountable PTAC HVAC System, 
                                                
                                                    Detached Garage Concrete Slab
                                                
                                                    Detached Garage Ext Brick Veneer Wall
                                                
                                                    Detached Garage Ext Metal Siding Wall
                                                
                                                    Detached Garage Ext Stone Veneer Wall
                                                
                                                    Detached Garage Ext Vinyl Siding Wall
                                                
                                                    Detached Garage Ext Wall Lighting
                                                
                                                    Detached Garage Ext Wall Lighting Electrical
                                                
                                                    Detached Garage Ext Wood Siding Wall
                                                
                                                    Detached Garage Ext Wood Stud Fiber Cement Wall
                                                
                                                    Detached Garage Ext Wood Stud Stucco Wall
                                                
                                                    Detached Garage Exterior Doors
                                                
                                                    Detached Garage General Lighting
                                                
                                                    Detached Garage Roof Cover
                                                
                                                    Detached Garage Roof Drainage
                                                
                                                    Detached Garage Roof Structure
                                                
                                                    Detached Outdoor Deck,
                                                
                                                    Dishwasher Drain,
                                                
                                                    Dishwasher Electrical,
                                                
                                                    Dishwasher Hood Electrical,
                                                
                                                    Dishwasher Water Supply
                                                
                                                    Elec Water Cooler
                                                
                                                    Electric Water Cooler
                                                
                                                    Electric Water Cooler Electrical
                                                
                                                    Elev Wood Joist Floor,
                                                
                                                    Elevated Floors
                                                
                                                    Elevator
                                                
                                                    Elevator Electrical
                                                
                                                    Elevator Sump
                                                
                                                    Elevator Sump Pump
                                                
                                                    Elevator,
                                                
                                                    Emergency
                                                
                                                    Emergency Generator Equipment,
                                                
                                                    Emergency Lighting
                                                
                                                    Employee Break Area Base
                                                
                                                    Employee Break Area Wall
                                                
                                                    Employee Lounge
                                                
                                                    Equip Elec Inc SVC
                                                
                                                    Equipment Elec
                                                
                                                    Equipment Electric
                                                
                                                    Equipment Electric SVC
                                                
                                                    Equipment Protection Bollards,
                                                
                                                    Equipment Protection Guardrails,
                                                
                                                    Exam Room Base Cabs w/Ctr
                                                
                                                    Exam Room Equipment
                                                
                                                    Exam Room Equipment Wash Sink
                                                
                                                    Exam Room Plumbing
                                                
                                                    Exam Room Sink
                                                
                                                    Exam Room Wall Cabinets
                                                
                                                    Exit
                                                
                                                    Exit &amp; Emerg Lt Combo
                                                
                                                    Exit Lighting
                                                
                                                    Ext Garage
                                                
                                                    Ext Stairs
                                                
                                                    Exterior
                                                
                                                    Exterior Canopy Can Lighting
                                                
                                                    Exterior Concrete Stairs
                                                
                                                    Exterior Doors
                                                
                                                    Exterior Garage
                                                
                                                    Exterior Staircase
                                                
                                                    Exterior Storage Bldg Doors
                                                
                                                    Exterior Storage Bldg Interior Partitions
                                                
                                                    Exterior Storage Bldg Structure,
                                                
                                                    Exterior Wall EIFS Coating Finish,
                                                
                                                    Exterior Wall Paint Finish,
                                                
                                                    Exterior Wall Stucco Finish,
                                                
                                                    Exterior Window Treatment,
                                                
                                                    Exterior Wood Stairs
                                                
                                                    Eye Wash Stations
                                                
                                                    FF &amp; E,
                                                
                                                    Fire Alarm, 
                                                
                                                    Fire Sprinkler System
                                                
                                                    Fireplace
                                                
                                                    Floating Floor - Common Area
                                                
                                                    Food Prep Kitchen
                                                
                                                    Food Prep Kitchen Base
                                                
                                                    Food Prep Kitchen Equipment
                                                
                                                    Food Prep Kitchen Sink
                                                
                                                    Food Prep Kitchen Wall
                                                
                                                    Front Desk w/Low Wall,
                                                
                                                    Front Entry
                                                
                                                    FRP Wall Covering,
                                                
                                                    Fryer Electrical,
                                                
                                                    Fume Hoods
                                                
                                                    Gas Dryer Electrical,
                                                
                                                    Gas Fryer
                                                
                                                    Gas Griddle
                                                
                                                    Gas Griddle &amp; Oven Combo
                                                
                                                    Gas Range
                                                
                                                    Gas Range &amp; Oven Combo
                                                
                                                    Gas Range, Griddle, &amp; Oven Combo
                                                
                                                    Gen Elec Inc SVC
                                                
                                                    Gen Gas WH
                                                
                                                    General
                                                
                                                    General Bathtub
                                                
                                                    General Domestic Water
                                                
                                                    General Electric
                                                
                                                    General Electric SVC
                                                
                                                    General Electric Water Heater
                                                
                                                    General Electric Water Htr
                                                
                                                    General Equipment Piping
                                                
                                                    General Gas
                                                
                                                    General Gas Pipe,
                                                
                                                    General Gas Water Heater
                                                
                                                    General Lavatory
                                                
                                                    General Lighting
                                                
                                                    General Lighting Electrical,
                                                
                                                    General San Sewer
                                                
                                                    General Sanitary
                                                
                                                    General Sanitary Sewer Pipe
                                                
                                                    General Sanitary Sewer Vent Pipe,
                                                
                                                    General Sanitary Sewer,
                                                
                                                    General Sanitary Vent,
                                                
                                                    General Shower
                                                
                                                    General SVC Sink
                                                
                                                    General Urinal
                                                
                                                    General Water
                                                
                                                    General Water Closet
                                                
                                                    General Water Heater
                                                
                                                    General Water Heater Gas Pipe,
                                                
                                                    General Water Heater Water Pipe,
                                                
                                                    General Water Pipe,
                                                
                                                    General WC
                                                
                                                    Griddle &amp; Oven Combo Electrical,
                                                
                                                    Griddle Electrical,
                                                
                                                    Guest Room Coffee Maker Electric
                                                
                                                    Guest Room Electrical
                                                
                                                    Guest Room Furniture,
                                                
                                                    Guest Room Hair Dryer Electric
                                                
                                                    Guest Room Microwave Electric
                                                
                                                    Guest Room Microwave,
                                                
                                                    Guest Room Mini Fridge Electric
                                                
                                                    Guest Room Mini Fridge,
                                                
                                                    Guest Room Refrigerator,
                                                
                                                    Guest Room Television,
                                                
                                                    Handicap Bathroom Accessories,
                                                
                                                    Handicap Lavatory,
                                                
                                                    Handicap Shower,
                                                
                                                    Handsink Drain
                                                
                                                    Handsink Water Supply
                                                
                                                    Hostess Station Counter,
                                                
                                                    Hot Tub Electrical
                                                
                                                    HVAC
                                                
                                                    HVAC AHU
                                                
                                                    HVAC Drain Piping,
                                                
                                                    HVAC Electric
                                                
                                                    HVAC Electric Unit Heater
                                                
                                                    HVAC Electrical
                                                
                                                    HVAC Gas Furnace
                                                
                                                    HVAC Gas Piping
                                                
                                                    HVAC Gas Piping,
                                                
                                                    HVAC RTU
                                                
                                                    HVAC Split System
                                                
                                                    HVAC Split System Elec Heater
                                                
                                                    HVAC Split System Rmt Condenser
                                                
                                                    HVAC SS
                                                
                                                    HVAC SS AHU
                                                
                                                    HVAC SS Rmt Cond
                                                
                                                    Hydraulic Equipment
                                                
                                                    Ice Cream Machine Electrical,
                                                
                                                    Inc Gen Elec SVC
                                                
                                                    Indoor
                                                
                                                    Indoor Coiling Door
                                                
                                                    Indoor Drinking Fountains,
                                                
                                                    Indoor Fireplace,
                                                
                                                    Indoor Hot Tub Electric Water Heater,
                                                
                                                    Indoor Hot Tub Equipment Electrical,
                                                
                                                    Indoor Hot Tub Gas Piping,
                                                
                                                    Indoor Hot Tub Gas Water Heater,
                                                
                                                    Indoor Hot Tub Water Piping,
                                                
                                                    Indoor Jacuzzi
                                                
                                                    Indoor Jacuzzi Equipment Electrical,
                                                
                                                    Indoor Jacuzzi Gas Piping,
                                                
                                                    Indoor Jacuzzi Heater,
                                                
                                                    Indoor Jacuzzi Piping,
                                                
                                                    Indoor Jacuzzi Water Supply Piping,
                                                
                                                    Indoor Jacuzzi,
                                                
                                                    Indoor Plant Wall Water
                                                
                                                    Indoor Pool Deck
                                                
                                                    Indoor Pool Dehumdifier,
                                                
                                                    Indoor Pool Dehumidifer,
                                                
                                                    Indoor Pool Equipment
                                                
                                                    Indoor Pool Equipment Electrical,
                                                
                                                    Indoor Pool Equipment Gas Piping,
                                                
                                                    Indoor Pool Exhaust
                                                
                                                    Indoor Pool Piping,
                                                
                                                    Indoor Pool Water Supply Piping,
                                                
                                                    Indoor Sauna Equipment
                                                
                                                    Indoor Sauna Equipment Electrical
                                                
                                                    Indoor Saunas
                                                
                                                    Indoor Steam Room Door,
                                                
                                                    Indoor Steam Room Drain Piping,
                                                
                                                    Indoor Steam Room Equipment
                                                
                                                    Indoor Steam Room Equipment Electrical
                                                
                                                    Indoor Steam Room Gas Piping,
                                                
                                                    Indoor Steam Room Piping
                                                
                                                    Indoor Steam Room Water Supply Piping,
                                                
                                                    Indoor Steam Rooms
                                                
                                                    Indoor Vertical Lift Door
                                                
                                                    Indoor Vertical Lift OHead
                                                
                                                    Int Inc Equip Elec SVC
                                                
                                                    Int Inc Gen Elec SVC
                                                
                                                    Int of Ext Wall Drywall Sheathing
                                                
                                                    Int of Exterior Wall
                                                
                                                    Int Sec Coiling Door,
                                                
                                                    Interior Closet
                                                
                                                    Interior Door
                                                
                                                    Interior Door,
                                                
                                                    Interior Doors
                                                
                                                    Interior Driveway
                                                
                                                    Interior Partition
                                                
                                                    Interior Partition,
                                                
                                                    Interior Partitions
                                                
                                                    Interior Perimeter Wall,
                                                
                                                    Interior Rollup Door,
                                                
                                                    Interior Staircase
                                                
                                                    Interior Storage Doors
                                                
                                                    Interior Storage Partitions
                                                
                                                    Interior Window Treatment,
                                                
                                                    Kitchen
                                                
                                                    Kitchen 3-Comp Sink
                                                
                                                    Kitchen Base Cabs
                                                
                                                    Kitchen Base Cabs w/Ctr
                                                
                                                    Kitchen Base Cabs,
                                                
                                                    Kitchen Built-In
                                                
                                                    Kitchen Counter
                                                
                                                    Kitchen Counter,
                                                
                                                    Kitchen Dishwasher Electric,
                                                
                                                    Kitchen Elec Range/Oven
                                                
                                                    Kitchen Equip Electrical
                                                
                                                    Kitchen Equipment
                                                
                                                    Kitchen Equipment Electric
                                                
                                                    Kitchen Equipment Electrical
                                                
                                                    Kitchen Exhaust
                                                
                                                    Kitchen Gas Piping,
                                                
                                                    Kitchen Gas Range
                                                
                                                    Kitchen Gas Range/Oven
                                                
                                                    Kitchen Gas Range/Oven,
                                                
                                                    Kitchen Main Gas
                                                
                                                    Kitchen Oven
                                                
                                                    Kitchen Range Electrical
                                                
                                                    Kitchen Range Electrical,
                                                
                                                    Kitchen Sink
                                                
                                                    Kitchen Sink,
                                                
                                                    Kitchen Wall
                                                
                                                    Kitchen Wall Cabs,
                                                
                                                    Lab Sink
                                                
                                                    Laboratory Base Cabs/w Ctrs,
                                                
                                                    Laboratory Electric
                                                
                                                    Laboratory Wall
                                                
                                                    Laminate
                                                
                                                    Laminate Flooring,
                                                
                                                    Landscape
                                                
                                                    Laundry
                                                
                                                    Laundry Base
                                                
                                                    Laundry Drain Piping,
                                                
                                                    Laundry Dryer,
                                                
                                                    Laundry Elec Dryer
                                                
                                                    Laundry Electric Dryer
                                                
                                                    Laundry Electric Water Heater
                                                
                                                    Laundry Equipment
                                                
                                                    Laundry Equipment Electrical
                                                
                                                    Laundry Gas Dryer
                                                
                                                    Laundry Gas Piping,
                                                
                                                    Laundry Piping
                                                
                                                    Laundry Sink
                                                
                                                    Laundry Wall
                                                
                                                    Laundry Washer
                                                
                                                    Laundry Washer,
                                                
                                                    Laundry Waste Pipe
                                                
                                                    Laundry Water Pipe
                                                
                                                    Laundry Water Piping,
                                                
                                                    LVT
                                                
                                                    Make-up Air Electrical,
                                                
                                                    Manufacturing Equipment Electrical,
                                                
                                                    Manufacturing Equipment Protection Bollards,
                                                
                                                    Manufacturing Equipment Protection Guardrails,
                                                
                                                    Manufacturing Equipment Water Piping,
                                                
                                                    Medical Equipment Electric
                                                
                                                    Medical Office Electric Water Heater
                                                
                                                    Medical Office Fixture Waste
                                                
                                                    Medical Office Fixture Water
                                                
                                                    Medical Office Gas Water Heater
                                                
                                                    Natural
                                                
                                                    Nurse's Station w/Low Wall Ctr
                                                
                                                    Nurses' Station Back Base Cabs w/Ctr
                                                
                                                    Nurses' Station Desk w/Low Wall Ctr
                                                
                                                    Nurses' Station Low Wall w.Ctr
                                                
                                                    Nurses' Station Sink
                                                
                                                    Nurses' Station Wall
                                                
                                                    Nurses' Station Wall Cabinet
                                                
                                                    Nurses' Station Wall Cabinets
                                                
                                                    Office Base Cabinetry,
                                                
                                                    Office Base Cabs w/Ctr
                                                
                                                    Office Furniture Electric
                                                
                                                    Office Wall
                                                
                                                    Office Wall Cabinetry,
                                                
                                                    Outdoor Deck,
                                                
                                                    Parking Garage
                                                
                                                    Parking Garage Bollards
                                                
                                                    Parking Garage CC Column
                                                
                                                    Parking Garage CC Footings
                                                
                                                    Parking Garage CC Slab
                                                
                                                    Parking Garage Drainage
                                                
                                                    Parking Garage Elev Concrete Floor
                                                
                                                    Parking Garage Elevator Electrical,
                                                
                                                    Parking Garage Elevator,
                                                
                                                    Parking Garage Ext Brick Wall
                                                
                                                    Parking Garage Ext CMU Stucco Wall
                                                
                                                    Parking Garage Ext CMU Wall
                                                
                                                    Parking Garage Ext Glass Curtain Wall
                                                
                                                    Parking Garage Ext Metal Panel Curtain Wall
                                                
                                                    Parking Garage Ext Stone Veneer Wall
                                                
                                                    Parking Garage Exterior Doors
                                                
                                                    Parking Garage Lighting
                                                
                                                    Parking Garage Sprinkler System
                                                
                                                    Parking Garage Steel Railing
                                                
                                                    Parking Garage Steel Railing,
                                                
                                                    Pier and Beam Foundation CC Footing,
                                                
                                                    Pier and Beam Foundation Excavation,
                                                
                                                    Pier and Beam Foundation Wood Columns,
                                                
                                                    Pier and Beam Wood Floor Construction,
                                                
                                                    PoS Equipment
                                                
                                                    Pot Sink Drain
                                                
                                                    Pot Sink Water Supply
                                                
                                                    Prescription Drug Refrigeration Equip
                                                
                                                    Process Area Electric Water Heater,
                                                
                                                    Process Area Gas Water Heater,
                                                
                                                    Process Area Roof Exhaust Electrical,
                                                
                                                    Process Area Roof Exhaust,
                                                
                                                    Process Area Trench Drains,
                                                
                                                    Process Area Wall Exhaust Electrical,
                                                
                                                    Process Area Wall Exhaust,
                                                
                                                    Process Area Water Heater Electrical,
                                                
                                                    Process Area Water Heater Piping,
                                                
                                                    PTAC HVAC Drain Piping,
                                                
                                                    PVC Wall Covering,
                                                
                                                    Range &amp; Oven Combo Electrical,
                                                
                                                    Range Electrical,
                                                
                                                    Range, Griddle, &amp; Oven Combo Electrical,
                                                
                                                    Reach-in Refrigerator Electrical,
                                                
                                                    Reception Back Base Cabinetry
                                                
                                                    Reception Back Wall Cabinetry
                                                
                                                    Reception Built-in Desk
                                                
                                                    Reception Desk Glass Wall,
                                                
                                                    Reception Desk w/Low Wall, 
                                                
                                                    Refrigeration Equipment
                                                
                                                    Refrigeration Room Electric,
                                                
                                                    Removable Mezzanine
                                                
                                                    Residential Unit
                                                
                                                    Restroom
                                                
                                                    Restroom Counter
                                                
                                                    Restroom Countertop,
                                                
                                                    Restroom Electric,
                                                
                                                    Restroom Exhaust
                                                
                                                    Restroom Hand Dryer
                                                
                                                    Restroom Lavatory,
                                                
                                                    Restroom Mirror
                                                
                                                    Restroom Urinal,
                                                
                                                    Restroom Vanity
                                                
                                                    Restroom Vanity Base Cabinets,
                                                
                                                    Restroom Vanity Base Cabs w/Ctr
                                                
                                                    Restroom Vanity Countertop,
                                                
                                                    Restroom Water Closet,
                                                
                                                    Roof Drain
                                                
                                                    Roof-Mtd Photovoltaic System
                                                
                                                    Sanitary
                                                
                                                    Security
                                                
                                                    Server Room HVAC
                                                
                                                    Sheet Vinyl,
                                                
                                                    Shower Enclosures,
                                                
                                                    Site
                                                
                                                    Site Asphalt
                                                
                                                    Site Asphalt Berm
                                                
                                                    Site Asphalt Driveway
                                                
                                                    Site Asphalt Paving
                                                
                                                    Site Barbecue Grille
                                                
                                                    Site Basketball
                                                
                                                    Site Bollards,
                                                
                                                    Site Brick Pavers
                                                
                                                    Site Carports,
                                                
                                                    Site CC Curb &amp; Gutter
                                                
                                                    Site CC Driveway
                                                
                                                    Site CC Patio
                                                
                                                    Site CC Sidewalk
                                                
                                                    Site CC Slab on Grade
                                                
                                                    Site CC Truck Ramp
                                                
                                                    Site Chain Link
                                                
                                                    Site Concrete Curb
                                                
                                                    Site Concrete Curb &amp; Gutter
                                                
                                                    Site Concrete Patio
                                                
                                                    Site Concrete Paving
                                                
                                                    Site Concrete Sidewalk
                                                
                                                    Site Concrete Walkways
                                                
                                                    Site Decorative Pole Lighting
                                                
                                                    Site Driveway Pavers
                                                
                                                    Site Dumpster Enclosure
                                                
                                                    Site Equipment Protection Bollards,
                                                
                                                    Site Fencing
                                                
                                                    Site Fire Hydrant
                                                
                                                    Site Flagpoles,
                                                
                                                    Site Gravel Driveway
                                                
                                                    Site Gravel Walkways
                                                
                                                    Site Guardrail,
                                                
                                                    Site Handrails,
                                                
                                                    Site Hot Tub
                                                
                                                    Site Hot Tub Equipment
                                                
                                                    Site Hot Tub Equipment Electrical
                                                
                                                    Site Hot Tub Equipment Piping
                                                
                                                    Site Hot Tub Pad
                                                
                                                    Site Incoming Electric SVC
                                                
                                                    Site Incoming Fire Protection SVC
                                                
                                                    Site Incoming N Gas SVC
                                                
                                                    Site Incoming San Sewer SVC
                                                
                                                    Site Incoming Water SVC
                                                
                                                    Site Irrigation
                                                
                                                    Site Jacuzzi
                                                
                                                    Site Jacuzzi Heater
                                                
                                                    Site Jacuzzi,
                                                
                                                    Site Landscape
                                                
                                                    Site Landscaping
                                                
                                                    Site Parking Curb,
                                                
                                                    Site Parking Curbs,
                                                
                                                    Site Parking Signs,
                                                
                                                    Site Patio Fencing,
                                                
                                                    Site Patio Pavers
                                                
                                                    Site Planter,
                                                
                                                    Site Pole Lighting
                                                
                                                    Site Porte Cochere Brick Veneer Finish
                                                
                                                    Site Porte Cochere Column Covering
                                                
                                                    Site Porte Cochere Concrete Footing
                                                
                                                    Site Porte Cochere EIFS Finish
                                                
                                                    Site Porte Cochere Lighting
                                                
                                                    Site Porte Cochere Roof Cover
                                                
                                                    Site Porte Cochere Roof Drainage
                                                
                                                    Site Porte Cochere Roof Structure
                                                
                                                    Site Porte Cochere Steel Beams
                                                
                                                    Site Porte Cochere Steel Columns
                                                
                                                    Site Porte Cochere Stone Veneer Finish
                                                
                                                    Site Porte Cochere Stucco Finish
                                                
                                                    Site Porte Cochere Wood Beams
                                                
                                                    Site Porte Cochere Wood Columns
                                                
                                                    Site Privacy Wall
                                                
                                                    Site Reinf CC Paving
                                                
                                                    Site Retaining Wall
                                                
                                                    Site Slate Pavers
                                                
                                                    Site Storm Drainage
                                                
                                                    Site Storm Water Detention Area
                                                
                                                    Site Swimming Pool
                                                
                                                    Site Swimming Pool Deck
                                                
                                                    Site Swimming Pool Enclosure
                                                
                                                    Site Swimming Pool Equipment
                                                
                                                    Site Swimming Pool Equipment Electrical
                                                
                                                    Site Swimming Pool Equipment Piping
                                                
                                                    Site Walkway Pavers
                                                
                                                    Site Water Line
                                                
                                                    Soiled Utility Sink
                                                
                                                    Sprinkler System
                                                
                                                    Stone Tile Finishing,
                                                
                                                    Storage Base Cabs w/Ctrs,
                                                
                                                    Storage Wall
                                                
                                                    Storefront Doors,
                                                
                                                    Storefront Windows,
                                                
                                                    Tall Storage Cabinets
                                                
                                                    Tenant Kitchen Spaces
                                                
                                                    Tenant Kitchen Spaces Gas WH
                                                
                                                    Tenant Space Finishes
                                                
                                                    Tenant Spaces Food Prep 3-Comp Sink
                                                
                                                    Terrace-Mounted Light Bollard
                                                
                                                    Vanity Base Cabinet,
                                                
                                                    Vanity Cabinet Countertop,
                                                
                                                    Vanity Countertop,
                                                
                                                    Vanity Sink,
                                                
                                                    VCT
                                                
                                                    VCT,
                                                
                                                    Vent Kit Electrical,
                                                
                                                    Vinyl Sheet Flooring,
                                                
                                                    Vinyl Wall Covering,
                                                
                                                    Walk-in Cooler Drain,
                                                
                                                    Walk-in Cooler Electric,
                                                
                                                    Walk-in Cooler,
                                                
                                                    Walk-in Freezer Drain,
                                                
                                                    Walk-in Freezer Electric,
                                                
                                                    Walk-in Freezer,
                                                
                                                    Walk-in Refrigerator Drain,
                                                
                                                    Walk-in Refrigerator Electrical,
                                                
                                                    Walk-in Refrigerator,
                                                
                                                    Wall
                                                
                                                    Wall Paneling,
                                                
                                                    Wall Tiles,
                                                
                                                    Wallpaper Wall Covering,
                                                
                                                    Wallpaper,
                                                
                                                    Water Distribution Piping,
                                                
                                                    Water Heater Electrical
                                                
                                                    Water Heater Gas Piping,
                                                
                                                    Water Heater Piping
                                                
                                                    Water Heater Piping,
                                                
                                                    Water Heater Water Piping,
                                                
                                                    Wet Bar
                                                
                                                    Wet Bar Sink
                                                
                                                    Window Shades
                                                
                                                    Window Treatment
                                                
                                                    Windows
                                                
                                                    Wood Flooring
                                                
                                                    Wood Flooring,
                                                
                                                    Wood Wall Paneling,
                                                
                                                    Work Room Base Cabs w/Ctr
                                                
                                                    Work Room Wall Cabinets
                                                
                                                    X-Ray Equipment
                                                
                                                    X-Ray Equipment Electrical
                                                
                                                    X-Ray Shielded Doors
                                                
                                                    X-Ray Shielded Partition
                                                
                                                    X-Ray Shielded Window
                                                
                                                    Add Additional Purpose Description Prefix
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                    
                                        
                                            Extra Purpose Description Prefix
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple Purpose Description Prefix
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Custom Items are not available for Purpose Description Prefixes. If you would like to change the description of any Custom Item, you may do so by double-clicking on the record for each Custom Item and changing the description inside of the dialogue that appears. The rest of your selection will be concatenated as desired.
                                        
                                        
                                            
                                                Update Purpose Description of  selected records.
                                            
                                            
                                                Purpose Description Prefix: 
                                            
                                            
                                                
                                                    None
                                                
                                                    A/V System
                                                
                                                    A/V System Equipment
                                                
                                                    A/V System Equipment Electric
                                                
                                                    A/V System,
                                                
                                                    Back Porch
                                                
                                                    Backsplash Wall
                                                
                                                    Balcony Exterior Doors,
                                                
                                                    Balcony Floor Finish,
                                                
                                                    Balcony Steel Railing,
                                                
                                                    Balcony Wall Light,
                                                
                                                    Base
                                                
                                                    Base Molding,
                                                
                                                    Basement Staircase,
                                                
                                                    Bldg
                                                
                                                    Bldg Basement
                                                
                                                    Bldg Basement Concrete Floor,
                                                
                                                    Bldg Basement Excavation, 
                                                
                                                    Bldg Basement FDN Wall
                                                
                                                    Bldg CC Column
                                                
                                                    Bldg CC Elevated Floor Structure
                                                
                                                    Bldg CC Footing
                                                
                                                    Bldg CC Footings
                                                
                                                    Bldg CC Slab
                                                
                                                    Bldg CC Slab on Grade
                                                
                                                    Bldg Column Fireproofing,
                                                
                                                    Bldg Concrete Ext Wall
                                                
                                                    Bldg Concrete Staircase,
                                                
                                                    Bldg Elev Beam FDN
                                                
                                                    Bldg Elev Concrete Floor
                                                
                                                    Bldg Elev Floor Structure
                                                
                                                    Bldg Elev Metal Joist Floor
                                                
                                                    Bldg Elev Structural Steel Floor
                                                
                                                    Bldg Elev Walkway
                                                
                                                    Bldg Elev Wood Joist Floor
                                                
                                                    Bldg Elevated Floor Structure
                                                
                                                    Bldg Ext Brick Stucco Wall
                                                
                                                    Bldg Ext Brick Veneer Wall
                                                
                                                    Bldg Ext Brick Wall
                                                
                                                    Bldg Ext CMU EIFS Wall
                                                
                                                    Bldg Ext CMU Stucco Wall
                                                
                                                    Bldg Ext CMU Wall
                                                
                                                    Bldg Ext Column Covering
                                                
                                                    Bldg Ext Concrete Wall Panels
                                                
                                                    Bldg Ext Demountable Canopy
                                                
                                                    Bldg Ext EIFS Wall
                                                
                                                    Bldg Ext Glass Curtain Wall
                                                
                                                    Bldg Ext Lighting
                                                
                                                    Bldg Ext Metal Panel Curtain Wall
                                                
                                                    Bldg Ext Metal Siding Wall
                                                
                                                    Bldg Ext Metal Stud EIFS Wall
                                                
                                                    Bldg Ext Metal Stud Fiber Cement Wall
                                                
                                                    Bldg Ext Metal Stud Stucco Wall
                                                
                                                    Bldg Ext Metal Stud Wall
                                                
                                                    Bldg Ext Stone Veneer Wall
                                                
                                                    Bldg Ext Tilt-Up CC Panel Wall
                                                
                                                    Bldg Ext Vinyl Siding Wall
                                                
                                                    Bldg Ext Wall
                                                
                                                    Bldg Ext Wall Lighting
                                                
                                                    Bldg Ext Wall Lighting Electrical
                                                
                                                    Bldg Ext Wood Siding Wall
                                                
                                                    Bldg Ext Wood Stud EIFS Wall
                                                
                                                    Bldg Ext Wood Stud Fiber Cement Wall
                                                
                                                    Bldg Ext Wood Stud Stucco Wall
                                                
                                                    Bldg Ext Wood Stud Wall
                                                
                                                    Bldg FDN
                                                
                                                    Bldg Masonry Ext Wall
                                                
                                                    Bldg Metal Ext Wall
                                                
                                                    Bldg PEMB Structure
                                                
                                                    Bldg Porte Cochere Brick Veneer Wall
                                                
                                                    Bldg Porte Cochere Column Covering
                                                
                                                    Bldg Porte Cochere Concrete Footing
                                                
                                                    Bldg Porte Cochere EIFS Wall
                                                
                                                    Bldg Porte Cochere Lighting
                                                
                                                    Bldg Porte Cochere Roof Cover
                                                
                                                    Bldg Porte Cochere Roof Drainage
                                                
                                                    Bldg Porte Cochere Roof Structure
                                                
                                                    Bldg Porte Cochere Steel Beams
                                                
                                                    Bldg Porte Cochere Steel Column
                                                
                                                    Bldg Porte Cochere Stone Veneer Wall
                                                
                                                    Bldg Porte Cochere Wood Beams
                                                
                                                    Bldg Porte Cochere Wood Columns
                                                
                                                    Bldg Porte Cochere Wood Stud Stucco Wall
                                                
                                                    Bldg Roof
                                                
                                                    Bldg Roof Cover
                                                
                                                    Bldg Roof Covering
                                                
                                                    Bldg Roof Drainage
                                                
                                                    Bldg Roof Structure
                                                
                                                    Bldg Staircase,
                                                
                                                    Bldg Steel Column
                                                
                                                    Bldg Steel Railing,
                                                
                                                    Bldg Steel Staircase,
                                                
                                                    Bldg Strip Foundation
                                                
                                                    Bldg Structural Steel Beam
                                                
                                                    Bldg Unit Concrete Staircase,
                                                
                                                    Bldg Unit Staircase,
                                                
                                                    Bldg Unit Steel Staircase,
                                                
                                                    Bldg Unit Wood Staircase,
                                                
                                                    Bldg Walkway Concrete Finishing,
                                                
                                                    Bldg Walkway Steel Railing,
                                                
                                                    Bldg Wd Elevated Floor Structure
                                                
                                                    Bldg Wood Column
                                                
                                                    Bldg Wood Columns
                                                
                                                    Bldg Wood Ext Wall
                                                
                                                    Bldg Wood Staircase,
                                                
                                                    Bldg-Mtd Lighting
                                                
                                                    Break Area
                                                
                                                    Break Area Base
                                                
                                                    Break Area Base Cabs w/Ctr
                                                
                                                    Break Area Base Cabs w/Ctr,
                                                
                                                    Break Area Dbl Sink
                                                
                                                    Break Area Dishwasher Electric,
                                                
                                                    Break Area Elec Water Heater
                                                
                                                    Break Area Equipment
                                                
                                                    Break Area Equipment Electrical
                                                
                                                    Break Area Gas Range/Oven
                                                
                                                    Break Area Sink
                                                
                                                    Break Area Wall
                                                
                                                    Break Area Wall Cabinets
                                                
                                                    Break Room Base Cabs w/Ctr
                                                
                                                    Break Room Equipment
                                                
                                                    Break Room Gas Range/Oven
                                                
                                                    Break Room Sink
                                                
                                                    Break Room Wall Cabinets
                                                
                                                    Breakfast Buffet Base
                                                
                                                    Breakfast Buffet Equipment
                                                
                                                    Building Foundation Wall
                                                
                                                    Building Spread Foundation
                                                
                                                    Building-Mounted
                                                
                                                    Building-Mounted Lighting
                                                
                                                    Built-In Desk,
                                                
                                                    Built-In Wardrobe,
                                                
                                                    CCTV Electrical,
                                                
                                                    Ceiling
                                                
                                                    Ceiling Fan Electrical,
                                                
                                                    Ceilings &amp; Partitions
                                                
                                                    Ceramic Tile
                                                
                                                    Chair Rail
                                                
                                                    Checkout Counter,
                                                
                                                    Chimney
                                                
                                                    Closet
                                                
                                                    Closet Door,
                                                
                                                    Cold Storage Ceiling Unit Cooler,
                                                
                                                    Cold Storage Condenser,
                                                
                                                    Cold Storage Drain Piping,
                                                
                                                    Cold Storage Electric,
                                                
                                                    Cold Storage Refrigeration Piping,
                                                
                                                    Cold Storage Remote Compressor,
                                                
                                                    Cold Storage,
                                                
                                                    Common
                                                
                                                    Common Area
                                                
                                                    Comms/Data Equipment
                                                
                                                    Comms/Data Equipment Electric
                                                
                                                    Compressed Air Equipment Electrical,
                                                
                                                    Compressed Air Equipment,
                                                
                                                    Compressed Air Piping,
                                                
                                                    Computer Equipment
                                                
                                                    Computer Equipment Electric
                                                
                                                    Computer Room HVAC
                                                
                                                    Concrete Topping Floor Finish
                                                
                                                    Condiments &amp; Beverage Counter,
                                                
                                                    Convenience Outlet,
                                                
                                                    Crane Equipment Electrical,
                                                
                                                    Crane Equipment,
                                                
                                                    Crane Rail Steel Support,
                                                
                                                    Crown Molding
                                                
                                                    Data Center Access Flooring, 
                                                
                                                    Data Center Electrical, 
                                                
                                                    Data Center Fire Suppression System, 
                                                
                                                    Data Center HVAC Electrical, 
                                                
                                                    Data Center HVAC, 
                                                
                                                    Data/Comms Equipment
                                                
                                                    Decorative
                                                
                                                    Decorative Pendant
                                                
                                                    Decorative Pendant Light
                                                
                                                    Decorative Pendant Lighting
                                                
                                                    Demountable Mezzanine
                                                
                                                    Demountable Partition, 
                                                
                                                    Demountable PTAC HVAC System Electrical, 
                                                
                                                    Demountable PTAC HVAC System, 
                                                
                                                    Detached Garage Concrete Slab
                                                
                                                    Detached Garage Ext Brick Veneer Wall
                                                
                                                    Detached Garage Ext Metal Siding Wall
                                                
                                                    Detached Garage Ext Stone Veneer Wall
                                                
                                                    Detached Garage Ext Vinyl Siding Wall
                                                
                                                    Detached Garage Ext Wall Lighting
                                                
                                                    Detached Garage Ext Wall Lighting Electrical
                                                
                                                    Detached Garage Ext Wood Siding Wall
                                                
                                                    Detached Garage Ext Wood Stud Fiber Cement Wall
                                                
                                                    Detached Garage Ext Wood Stud Stucco Wall
                                                
                                                    Detached Garage Exterior Doors
                                                
                                                    Detached Garage General Lighting
                                                
                                                    Detached Garage Roof Cover
                                                
                                                    Detached Garage Roof Drainage
                                                
                                                    Detached Garage Roof Structure
                                                
                                                    Detached Outdoor Deck,
                                                
                                                    Dishwasher Drain,
                                                
                                                    Dishwasher Electrical,
                                                
                                                    Dishwasher Hood Electrical,
                                                
                                                    Dishwasher Water Supply
                                                
                                                    Elec Water Cooler
                                                
                                                    Electric Water Cooler
                                                
                                                    Electric Water Cooler Electrical
                                                
                                                    Elev Wood Joist Floor,
                                                
                                                    Elevated Floors
                                                
                                                    Elevator
                                                
                                                    Elevator Electrical
                                                
                                                    Elevator Sump
                                                
                                                    Elevator Sump Pump
                                                
                                                    Elevator,
                                                
                                                    Emergency
                                                
                                                    Emergency Generator Equipment,
                                                
                                                    Emergency Lighting
                                                
                                                    Employee Break Area Base
                                                
                                                    Employee Break Area Wall
                                                
                                                    Employee Lounge
                                                
                                                    Equip Elec Inc SVC
                                                
                                                    Equipment Elec
                                                
                                                    Equipment Electric
                                                
                                                    Equipment Electric SVC
                                                
                                                    Equipment Protection Bollards,
                                                
                                                    Equipment Protection Guardrails,
                                                
                                                    Exam Room Base Cabs w/Ctr
                                                
                                                    Exam Room Equipment
                                                
                                                    Exam Room Equipment Wash Sink
                                                
                                                    Exam Room Plumbing
                                                
                                                    Exam Room Sink
                                                
                                                    Exam Room Wall Cabinets
                                                
                                                    Exit
                                                
                                                    Exit &amp; Emerg Lt Combo
                                                
                                                    Exit Lighting
                                                
                                                    Ext Garage
                                                
                                                    Ext Stairs
                                                
                                                    Exterior
                                                
                                                    Exterior Canopy Can Lighting
                                                
                                                    Exterior Concrete Stairs
                                                
                                                    Exterior Doors
                                                
                                                    Exterior Garage
                                                
                                                    Exterior Staircase
                                                
                                                    Exterior Storage Bldg Doors
                                                
                                                    Exterior Storage Bldg Interior Partitions
                                                
                                                    Exterior Storage Bldg Structure,
                                                
                                                    Exterior Wall EIFS Coating Finish,
                                                
                                                    Exterior Wall Paint Finish,
                                                
                                                    Exterior Wall Stucco Finish,
                                                
                                                    Exterior Window Treatment,
                                                
                                                    Exterior Wood Stairs
                                                
                                                    Eye Wash Stations
                                                
                                                    FF &amp; E,
                                                
                                                    Fire Alarm, 
                                                
                                                    Fire Sprinkler System
                                                
                                                    Fireplace
                                                
                                                    Floating Floor - Common Area
                                                
                                                    Food Prep Kitchen
                                                
                                                    Food Prep Kitchen Base
                                                
                                                    Food Prep Kitchen Equipment
                                                
                                                    Food Prep Kitchen Sink
                                                
                                                    Food Prep Kitchen Wall
                                                
                                                    Front Desk w/Low Wall,
                                                
                                                    Front Entry
                                                
                                                    FRP Wall Covering,
                                                
                                                    Fryer Electrical,
                                                
                                                    Fume Hoods
                                                
                                                    Gas Dryer Electrical,
                                                
                                                    Gas Fryer
                                                
                                                    Gas Griddle
                                                
                                                    Gas Griddle &amp; Oven Combo
                                                
                                                    Gas Range
                                                
                                                    Gas Range &amp; Oven Combo
                                                
                                                    Gas Range, Griddle, &amp; Oven Combo
                                                
                                                    Gen Elec Inc SVC
                                                
                                                    Gen Gas WH
                                                
                                                    General
                                                
                                                    General Bathtub
                                                
                                                    General Domestic Water
                                                
                                                    General Electric
                                                
                                                    General Electric SVC
                                                
                                                    General Electric Water Heater
                                                
                                                    General Electric Water Htr
                                                
                                                    General Equipment Piping
                                                
                                                    General Gas
                                                
                                                    General Gas Pipe,
                                                
                                                    General Gas Water Heater
                                                
                                                    General Lavatory
                                                
                                                    General Lighting
                                                
                                                    General Lighting Electrical,
                                                
                                                    General San Sewer
                                                
                                                    General Sanitary
                                                
                                                    General Sanitary Sewer Pipe
                                                
                                                    General Sanitary Sewer Vent Pipe,
                                                
                                                    General Sanitary Sewer,
                                                
                                                    General Sanitary Vent,
                                                
                                                    General Shower
                                                
                                                    General SVC Sink
                                                
                                                    General Urinal
                                                
                                                    General Water
                                                
                                                    General Water Closet
                                                
                                                    General Water Heater
                                                
                                                    General Water Heater Gas Pipe,
                                                
                                                    General Water Heater Water Pipe,
                                                
                                                    General Water Pipe,
                                                
                                                    General WC
                                                
                                                    Griddle &amp; Oven Combo Electrical,
                                                
                                                    Griddle Electrical,
                                                
                                                    Guest Room Coffee Maker Electric
                                                
                                                    Guest Room Electrical
                                                
                                                    Guest Room Furniture,
                                                
                                                    Guest Room Hair Dryer Electric
                                                
                                                    Guest Room Microwave Electric
                                                
                                                    Guest Room Microwave,
                                                
                                                    Guest Room Mini Fridge Electric
                                                
                                                    Guest Room Mini Fridge,
                                                
                                                    Guest Room Refrigerator,
                                                
                                                    Guest Room Television,
                                                
                                                    Handicap Bathroom Accessories,
                                                
                                                    Handicap Lavatory,
                                                
                                                    Handicap Shower,
                                                
                                                    Handsink Drain
                                                
                                                    Handsink Water Supply
                                                
                                                    Hostess Station Counter,
                                                
                                                    Hot Tub Electrical
                                                
                                                    HVAC
                                                
                                                    HVAC AHU
                                                
                                                    HVAC Drain Piping,
                                                
                                                    HVAC Electric
                                                
                                                    HVAC Electric Unit Heater
                                                
                                                    HVAC Electrical
                                                
                                                    HVAC Gas Furnace
                                                
                                                    HVAC Gas Piping
                                                
                                                    HVAC Gas Piping,
                                                
                                                    HVAC RTU
                                                
                                                    HVAC Split System
                                                
                                                    HVAC Split System Elec Heater
                                                
                                                    HVAC Split System Rmt Condenser
                                                
                                                    HVAC SS
                                                
                                                    HVAC SS AHU
                                                
                                                    HVAC SS Rmt Cond
                                                
                                                    Hydraulic Equipment
                                                
                                                    Ice Cream Machine Electrical,
                                                
                                                    Inc Gen Elec SVC
                                                
                                                    Indoor
                                                
                                                    Indoor Coiling Door
                                                
                                                    Indoor Drinking Fountains,
                                                
                                                    Indoor Fireplace,
                                                
                                                    Indoor Hot Tub Electric Water Heater,
                                                
                                                    Indoor Hot Tub Equipment Electrical,
                                                
                                                    Indoor Hot Tub Gas Piping,
                                                
                                                    Indoor Hot Tub Gas Water Heater,
                                                
                                                    Indoor Hot Tub Water Piping,
                                                
                                                    Indoor Jacuzzi
                                                
                                                    Indoor Jacuzzi Equipment Electrical,
                                                
                                                    Indoor Jacuzzi Gas Piping,
                                                
                                                    Indoor Jacuzzi Heater,
                                                
                                                    Indoor Jacuzzi Piping,
                                                
                                                    Indoor Jacuzzi Water Supply Piping,
                                                
                                                    Indoor Jacuzzi,
                                                
                                                    Indoor Plant Wall Water
                                                
                                                    Indoor Pool Deck
                                                
                                                    Indoor Pool Dehumdifier,
                                                
                                                    Indoor Pool Dehumidifer,
                                                
                                                    Indoor Pool Equipment
                                                
                                                    Indoor Pool Equipment Electrical,
                                                
                                                    Indoor Pool Equipment Gas Piping,
                                                
                                                    Indoor Pool Exhaust
                                                
                                                    Indoor Pool Piping,
                                                
                                                    Indoor Pool Water Supply Piping,
                                                
                                                    Indoor Sauna Equipment
                                                
                                                    Indoor Sauna Equipment Electrical
                                                
                                                    Indoor Saunas
                                                
                                                    Indoor Steam Room Door,
                                                
                                                    Indoor Steam Room Drain Piping,
                                                
                                                    Indoor Steam Room Equipment
                                                
                                                    Indoor Steam Room Equipment Electrical
                                                
                                                    Indoor Steam Room Gas Piping,
                                                
                                                    Indoor Steam Room Piping
                                                
                                                    Indoor Steam Room Water Supply Piping,
                                                
                                                    Indoor Steam Rooms
                                                
                                                    Indoor Vertical Lift Door
                                                
                                                    Indoor Vertical Lift OHead
                                                
                                                    Int Inc Equip Elec SVC
                                                
                                                    Int Inc Gen Elec SVC
                                                
                                                    Int of Ext Wall Drywall Sheathing
                                                
                                                    Int of Exterior Wall
                                                
                                                    Int Sec Coiling Door,
                                                
                                                    Interior Closet
                                                
                                                    Interior Door
                                                
                                                    Interior Door,
                                                
                                                    Interior Doors
                                                
                                                    Interior Driveway
                                                
                                                    Interior Partition
                                                
                                                    Interior Partition,
                                                
                                                    Interior Partitions
                                                
                                                    Interior Perimeter Wall,
                                                
                                                    Interior Rollup Door,
                                                
                                                    Interior Staircase
                                                
                                                    Interior Storage Doors
                                                
                                                    Interior Storage Partitions
                                                
                                                    Interior Window Treatment,
                                                
                                                    Kitchen
                                                
                                                    Kitchen 3-Comp Sink
                                                
                                                    Kitchen Base Cabs
                                                
                                                    Kitchen Base Cabs w/Ctr
                                                
                                                    Kitchen Base Cabs,
                                                
                                                    Kitchen Built-In
                                                
                                                    Kitchen Counter
                                                
                                                    Kitchen Counter,
                                                
                                                    Kitchen Dishwasher Electric,
                                                
                                                    Kitchen Elec Range/Oven
                                                
                                                    Kitchen Equip Electrical
                                                
                                                    Kitchen Equipment
                                                
                                                    Kitchen Equipment Electric
                                                
                                                    Kitchen Equipment Electrical
                                                
                                                    Kitchen Exhaust
                                                
                                                    Kitchen Gas Piping,
                                                
                                                    Kitchen Gas Range
                                                
                                                    Kitchen Gas Range/Oven
                                                
                                                    Kitchen Gas Range/Oven,
                                                
                                                    Kitchen Main Gas
                                                
                                                    Kitchen Oven
                                                
                                                    Kitchen Range Electrical
                                                
                                                    Kitchen Range Electrical,
                                                
                                                    Kitchen Sink
                                                
                                                    Kitchen Sink,
                                                
                                                    Kitchen Wall
                                                
                                                    Kitchen Wall Cabs,
                                                
                                                    Lab Sink
                                                
                                                    Laboratory Base Cabs/w Ctrs,
                                                
                                                    Laboratory Electric
                                                
                                                    Laboratory Wall
                                                
                                                    Laminate
                                                
                                                    Laminate Flooring,
                                                
                                                    Landscape
                                                
                                                    Laundry
                                                
                                                    Laundry Base
                                                
                                                    Laundry Drain Piping,
                                                
                                                    Laundry Dryer,
                                                
                                                    Laundry Elec Dryer
                                                
                                                    Laundry Electric Dryer
                                                
                                                    Laundry Electric Water Heater
                                                
                                                    Laundry Equipment
                                                
                                                    Laundry Equipment Electrical
                                                
                                                    Laundry Gas Dryer
                                                
                                                    Laundry Gas Piping,
                                                
                                                    Laundry Piping
                                                
                                                    Laundry Sink
                                                
                                                    Laundry Wall
                                                
                                                    Laundry Washer
                                                
                                                    Laundry Washer,
                                                
                                                    Laundry Waste Pipe
                                                
                                                    Laundry Water Pipe
                                                
                                                    Laundry Water Piping,
                                                
                                                    LVT
                                                
                                                    Make-up Air Electrical,
                                                
                                                    Manufacturing Equipment Electrical,
                                                
                                                    Manufacturing Equipment Protection Bollards,
                                                
                                                    Manufacturing Equipment Protection Guardrails,
                                                
                                                    Manufacturing Equipment Water Piping,
                                                
                                                    Medical Equipment Electric
                                                
                                                    Medical Office Electric Water Heater
                                                
                                                    Medical Office Fixture Waste
                                                
                                                    Medical Office Fixture Water
                                                
                                                    Medical Office Gas Water Heater
                                                
                                                    Natural
                                                
                                                    Nurse's Station w/Low Wall Ctr
                                                
                                                    Nurses' Station Back Base Cabs w/Ctr
                                                
                                                    Nurses' Station Desk w/Low Wall Ctr
                                                
                                                    Nurses' Station Low Wall w.Ctr
                                                
                                                    Nurses' Station Sink
                                                
                                                    Nurses' Station Wall
                                                
                                                    Nurses' Station Wall Cabinet
                                                
                                                    Nurses' Station Wall Cabinets
                                                
                                                    Office Base Cabinetry,
                                                
                                                    Office Base Cabs w/Ctr
                                                
                                                    Office Furniture Electric
                                                
                                                    Office Wall
                                                
                                                    Office Wall Cabinetry,
                                                
                                                    Outdoor Deck,
                                                
                                                    Parking Garage
                                                
                                                    Parking Garage Bollards
                                                
                                                    Parking Garage CC Column
                                                
                                                    Parking Garage CC Footings
                                                
                                                    Parking Garage CC Slab
                                                
                                                    Parking Garage Drainage
                                                
                                                    Parking Garage Elev Concrete Floor
                                                
                                                    Parking Garage Elevator Electrical,
                                                
                                                    Parking Garage Elevator,
                                                
                                                    Parking Garage Ext Brick Wall
                                                
                                                    Parking Garage Ext CMU Stucco Wall
                                                
                                                    Parking Garage Ext CMU Wall
                                                
                                                    Parking Garage Ext Glass Curtain Wall
                                                
                                                    Parking Garage Ext Metal Panel Curtain Wall
                                                
                                                    Parking Garage Ext Stone Veneer Wall
                                                
                                                    Parking Garage Exterior Doors
                                                
                                                    Parking Garage Lighting
                                                
                                                    Parking Garage Sprinkler System
                                                
                                                    Parking Garage Steel Railing
                                                
                                                    Parking Garage Steel Railing,
                                                
                                                    Pier and Beam Foundation CC Footing,
                                                
                                                    Pier and Beam Foundation Excavation,
                                                
                                                    Pier and Beam Foundation Wood Columns,
                                                
                                                    Pier and Beam Wood Floor Construction,
                                                
                                                    PoS Equipment
                                                
                                                    Pot Sink Drain
                                                
                                                    Pot Sink Water Supply
                                                
                                                    Prescription Drug Refrigeration Equip
                                                
                                                    Process Area Electric Water Heater,
                                                
                                                    Process Area Gas Water Heater,
                                                
                                                    Process Area Roof Exhaust Electrical,
                                                
                                                    Process Area Roof Exhaust,
                                                
                                                    Process Area Trench Drains,
                                                
                                                    Process Area Wall Exhaust Electrical,
                                                
                                                    Process Area Wall Exhaust,
                                                
                                                    Process Area Water Heater Electrical,
                                                
                                                    Process Area Water Heater Piping,
                                                
                                                    PTAC HVAC Drain Piping,
                                                
                                                    PVC Wall Covering,
                                                
                                                    Range &amp; Oven Combo Electrical,
                                                
                                                    Range Electrical,
                                                
                                                    Range, Griddle, &amp; Oven Combo Electrical,
                                                
                                                    Reach-in Refrigerator Electrical,
                                                
                                                    Reception Back Base Cabinetry
                                                
                                                    Reception Back Wall Cabinetry
                                                
                                                    Reception Built-in Desk
                                                
                                                    Reception Desk Glass Wall,
                                                
                                                    Reception Desk w/Low Wall, 
                                                
                                                    Refrigeration Equipment
                                                
                                                    Refrigeration Room Electric,
                                                
                                                    Removable Mezzanine
                                                
                                                    Residential Unit
                                                
                                                    Restroom
                                                
                                                    Restroom Counter
                                                
                                                    Restroom Countertop,
                                                
                                                    Restroom Electric,
                                                
                                                    Restroom Exhaust
                                                
                                                    Restroom Hand Dryer
                                                
                                                    Restroom Lavatory,
                                                
                                                    Restroom Mirror
                                                
                                                    Restroom Urinal,
                                                
                                                    Restroom Vanity
                                                
                                                    Restroom Vanity Base Cabinets,
                                                
                                                    Restroom Vanity Base Cabs w/Ctr
                                                
                                                    Restroom Vanity Countertop,
                                                
                                                    Restroom Water Closet,
                                                
                                                    Roof Drain
                                                
                                                    Roof-Mtd Photovoltaic System
                                                
                                                    Sanitary
                                                
                                                    Security
                                                
                                                    Server Room HVAC
                                                
                                                    Sheet Vinyl,
                                                
                                                    Shower Enclosures,
                                                
                                                    Site
                                                
                                                    Site Asphalt
                                                
                                                    Site Asphalt Berm
                                                
                                                    Site Asphalt Driveway
                                                
                                                    Site Asphalt Paving
                                                
                                                    Site Barbecue Grille
                                                
                                                    Site Basketball
                                                
                                                    Site Bollards,
                                                
                                                    Site Brick Pavers
                                                
                                                    Site Carports,
                                                
                                                    Site CC Curb &amp; Gutter
                                                
                                                    Site CC Driveway
                                                
                                                    Site CC Patio
                                                
                                                    Site CC Sidewalk
                                                
                                                    Site CC Slab on Grade
                                                
                                                    Site CC Truck Ramp
                                                
                                                    Site Chain Link
                                                
                                                    Site Concrete Curb
                                                
                                                    Site Concrete Curb &amp; Gutter
                                                
                                                    Site Concrete Patio
                                                
                                                    Site Concrete Paving
                                                
                                                    Site Concrete Sidewalk
                                                
                                                    Site Concrete Walkways
                                                
                                                    Site Decorative Pole Lighting
                                                
                                                    Site Driveway Pavers
                                                
                                                    Site Dumpster Enclosure
                                                
                                                    Site Equipment Protection Bollards,
                                                
                                                    Site Fencing
                                                
                                                    Site Fire Hydrant
                                                
                                                    Site Flagpoles,
                                                
                                                    Site Gravel Driveway
                                                
                                                    Site Gravel Walkways
                                                
                                                    Site Guardrail,
                                                
                                                    Site Handrails,
                                                
                                                    Site Hot Tub
                                                
                                                    Site Hot Tub Equipment
                                                
                                                    Site Hot Tub Equipment Electrical
                                                
                                                    Site Hot Tub Equipment Piping
                                                
                                                    Site Hot Tub Pad
                                                
                                                    Site Incoming Electric SVC
                                                
                                                    Site Incoming Fire Protection SVC
                                                
                                                    Site Incoming N Gas SVC
                                                
                                                    Site Incoming San Sewer SVC
                                                
                                                    Site Incoming Water SVC
                                                
                                                    Site Irrigation
                                                
                                                    Site Jacuzzi
                                                
                                                    Site Jacuzzi Heater
                                                
                                                    Site Jacuzzi,
                                                
                                                    Site Landscape
                                                
                                                    Site Landscaping
                                                
                                                    Site Parking Curb,
                                                
                                                    Site Parking Curbs,
                                                
                                                    Site Parking Signs,
                                                
                                                    Site Patio Fencing,
                                                
                                                    Site Patio Pavers
                                                
                                                    Site Planter,
                                                
                                                    Site Pole Lighting
                                                
                                                    Site Porte Cochere Brick Veneer Finish
                                                
                                                    Site Porte Cochere Column Covering
                                                
                                                    Site Porte Cochere Concrete Footing
                                                
                                                    Site Porte Cochere EIFS Finish
                                                
                                                    Site Porte Cochere Lighting
                                                
                                                    Site Porte Cochere Roof Cover
                                                
                                                    Site Porte Cochere Roof Drainage
                                                
                                                    Site Porte Cochere Roof Structure
                                                
                                                    Site Porte Cochere Steel Beams
                                                
                                                    Site Porte Cochere Steel Columns
                                                
                                                    Site Porte Cochere Stone Veneer Finish
                                                
                                                    Site Porte Cochere Stucco Finish
                                                
                                                    Site Porte Cochere Wood Beams
                                                
                                                    Site Porte Cochere Wood Columns
                                                
                                                    Site Privacy Wall
                                                
                                                    Site Reinf CC Paving
                                                
                                                    Site Retaining Wall
                                                
                                                    Site Slate Pavers
                                                
                                                    Site Storm Drainage
                                                
                                                    Site Storm Water Detention Area
                                                
                                                    Site Swimming Pool
                                                
                                                    Site Swimming Pool Deck
                                                
                                                    Site Swimming Pool Enclosure
                                                
                                                    Site Swimming Pool Equipment
                                                
                                                    Site Swimming Pool Equipment Electrical
                                                
                                                    Site Swimming Pool Equipment Piping
                                                
                                                    Site Walkway Pavers
                                                
                                                    Site Water Line
                                                
                                                    Soiled Utility Sink
                                                
                                                    Sprinkler System
                                                
                                                    Stone Tile Finishing,
                                                
                                                    Storage Base Cabs w/Ctrs,
                                                
                                                    Storage Wall
                                                
                                                    Storefront Doors,
                                                
                                                    Storefront Windows,
                                                
                                                    Tall Storage Cabinets
                                                
                                                    Tenant Kitchen Spaces
                                                
                                                    Tenant Kitchen Spaces Gas WH
                                                
                                                    Tenant Space Finishes
                                                
                                                    Tenant Spaces Food Prep 3-Comp Sink
                                                
                                                    Terrace-Mounted Light Bollard
                                                
                                                    Vanity Base Cabinet,
                                                
                                                    Vanity Cabinet Countertop,
                                                
                                                    Vanity Countertop,
                                                
                                                    Vanity Sink,
                                                
                                                    VCT
                                                
                                                    VCT,
                                                
                                                    Vent Kit Electrical,
                                                
                                                    Vinyl Sheet Flooring,
                                                
                                                    Vinyl Wall Covering,
                                                
                                                    Walk-in Cooler Drain,
                                                
                                                    Walk-in Cooler Electric,
                                                
                                                    Walk-in Cooler,
                                                
                                                    Walk-in Freezer Drain,
                                                
                                                    Walk-in Freezer Electric,
                                                
                                                    Walk-in Freezer,
                                                
                                                    Walk-in Refrigerator Drain,
                                                
                                                    Walk-in Refrigerator Electrical,
                                                
                                                    Walk-in Refrigerator,
                                                
                                                    Wall
                                                
                                                    Wall Paneling,
                                                
                                                    Wall Tiles,
                                                
                                                    Wallpaper Wall Covering,
                                                
                                                    Wallpaper,
                                                
                                                    Water Distribution Piping,
                                                
                                                    Water Heater Electrical
                                                
                                                    Water Heater Gas Piping,
                                                
                                                    Water Heater Piping
                                                
                                                    Water Heater Piping,
                                                
                                                    Water Heater Water Piping,
                                                
                                                    Wet Bar
                                                
                                                    Wet Bar Sink
                                                
                                                    Window Shades
                                                
                                                    Window Treatment
                                                
                                                    Windows
                                                
                                                    Wood Flooring
                                                
                                                    Wood Flooring,
                                                
                                                    Wood Wall Paneling,
                                                
                                                    Work Room Base Cabs w/Ctr
                                                
                                                    Work Room Wall Cabinets
                                                
                                                    X-Ray Equipment
                                                
                                                    X-Ray Equipment Electrical
                                                
                                                    X-Ray Shielded Doors
                                                
                                                    X-Ray Shielded Partition
                                                
                                                    X-Ray Shielded Window
                                                
                                                    Add Additional Purpose Description Prefix
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                    
                                        
                                            Extra Purpose Description Prefix
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Location
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Location of  selected records.
                                        
                                        
                                            Location: 
                                        
                                        
                                            
                                                Select
                                            
                                                Building
                                            
                                                Building Interior
                                            
                                                Site Exterior
                                            
                                                Add Additional Location
                                            
                                            
                                            
                                            
                                        
                                    
                                    
                                        
                                            Building
                                        
                                        
                                            
                                                
                                            
                                            
                                        
                                    
                                    
                                        
                                            Extra Location
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                
                
                    ×
                     Import Dynamic Attribute
                
                
                    
                        
                            Upload File
                        
                        
                            
                            
                        
                    
                
            
            
                Upload
                Cancel
            
        
    



    
        
            
                ×
                  Warning
            
            
                
                    
                
            
        
    



    
    
        
            
                
                    ×
                     Import Previous Assets
                    
                    
                        
                            
                                Select Property:
                            
                            
                                
                                    Select
                                    
                                
                            
                        
                    
                    
                        
                            
                                Select Asset
                            
                            
                                
                                
                            
                        
                    
                
                
                    Import
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update Location Multiplier
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Location Multiplier of  selected records.
                                        
                                        
                                            Location: 
                                        
                                        
                                            Location Multiplier: 
                                        
                                        
                                        
                                        
                                    
                                    
                                        
                                           
                                            Select
                                            
                                           
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $('#location_list').on('change', function(e){
        var location_modifier = $(this).val();
        var building_id = $('option:selected', this).attr('data-building_id');
        var location = $('option:selected', this).attr('data-location');

        $('#building_location').val(building_id);
        $('#location_id').val(location);
        $('#location_modifier').prop('disabled', false).val(location_modifier);

    })
    $(&quot;#change_location_modifier_btn&quot;).on(&quot;click&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $(&quot;.location_modifier&quot;).each(function(){
            $(this).closest(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);


            if(!$(this).val()){
                $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a value&quot;)
                $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);

                valid=false
            }
            else{
                if(isNaN(parseFloat($(this).val()))){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
                else if(parseFloat($(&quot;.location_modifier&quot;).val()) &lt;= 0){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
            }

        })
        if (valid){
            $(&quot;#location_modifier_form&quot;).submit();
        }
    })

    function location_modifier_clear_validation(){
        $(&quot;.location_modifier_error&quot;).text(&quot;&quot;)
        $(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);
    }


    
    
        
            
                ×
                Map Location
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Locations in use
                                            
                                        
                                        
                                            
                                                Update Location
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Location
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select New Location
                                                
                                                    Building
                                                
                                                    Building Interior
                                                
                                                    Site Exterior
                                                
                                                Add Additional Location
                                            
                                            
                                        

                                        
                                            
                                                Building
                                            
                                            
                                                Select
                                                
                                            
                                            
                                        

                                        
                                            Extra Location
                                            
                                            
                                        
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_map_location_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select-location').each(function(){
            if(!$(this).val()){
                $(this).siblings(&quot;.location_error&quot;).text(&quot;Please Select a location&quot;);
                valid=false
            }
        });

        if (valid){
            $(&quot;#update_map_location_form&quot;).submit();
        }
    });

    $(document).on('change', '#location_obj_id', function(e){
        var building_id = $('option:selected', this).attr('data-building_id');

        $('#old_building_id').val(building_id)
    });

    
    
    
        
            
                ×
                Map Takeoff Cost
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Takeoff Cost to be Replaced
                                            
                                        
                                        
                                            
                                                Replace With
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select
                                                
                                                None
                                            
                                            
                                        

                                        
                                            
                                                Select
                                                
                                                None
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#map_tc_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select-tc').each(function(){
            if($(this).val() == null){
                $(this).siblings(&quot;.tc_error&quot;).text(&quot;Please Select a Takeoff cost&quot;);
                valid=false
            }
        });

        if (valid){
            $(&quot;#map_tc_form&quot;).submit();
        }
    });

    
    
        
            
                ×
                Validation Warning!
            
            
                Your Response to this Question is outside of the Recommended Range, what would you like to do?
            
            
                
                    Use Current Response
                
                
                    Edit my Response
                
            
        
    

    
    
        
            
                 ×
                Notification
            
            
                The  Worksheet is Now Ready to Load.

                What would you like to do?
            
            
                Enter the Worksheet Now
                Continue With What I'm Currently Doing
            
        
    

    
    
        
            
                ×
                Map Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Recovery Period in use
                                            
                                        
                                        
                                            
                                                Update Recovery Period
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Recovery Period
                                                
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select New Recovery Period
                                                
                                                    
                                                        5 Years
                                                    
                                                
                                                    
                                                        15 Years
                                                    
                                                
                                                    
                                                        39 Years
                                                    
                                                
                                                    
                                                
                                                Soft Cost
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_recovery_period_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select_recovery_period').each(function(){

            if(!$(this).val()){
                $(this).siblings(&quot;.recovery_period_error&quot;).text(&quot;Please Select Recovery Period&quot;);
                valid=false
            }
        });

        if (valid){
            $(&quot;#update_recovery_period_form&quot;).submit();
        }
    });

    
    
        
            
                ×
                Map Purpose Description to Asset Classes
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Map Purpose Description
                                            
                                        
                                        
                                            
                                                Update Asset Class
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Purpose Description
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select Asset Class
                                                
                                                        1 - GENERAL REQUIREMENTS
                                                
                                                        2 - SITEWORK
                                                
                                                        3 - CONCRETE
                                                
                                                        4 - MASONRY
                                                
                                                        5 - METALS
                                                
                                                        6 - WOOD &amp; PLASTICS
                                                
                                                        7 - THERMAL &amp; MOISTURE PROTECTION
                                                
                                                        8 - DOORS &amp; WINDOWS
                                                
                                                        9 - FINISHES
                                                
                                                        10 - SPECIALTIES
                                                
                                                        11 - EQUIPMENT
                                                
                                                        12 - FURNISHINGS
                                                
                                                        13 - SPECIAL CONSTRUCTION
                                                
                                                        14 - CONVEYING SYSTEMS
                                                
                                                        15 - PLUMBING
                                                
                                                        16 - MECHANICAL
                                                
                                                        17 - HVAC
                                                
                                                        18 - ELECTRICAL
                                                
                                                        19 - BUILDING GAS SYSTEM
                                                
                                                        20 - BUILDING FIRE PROTECTION
                                                
                                                        21 - SITE FIRE PROTECTION
                                                
                                                        30 - ATM ELECTRICAL
                                                
                                                        32 - AUDIO/VISUAL SYSTEM
                                                
                                                        34 - AUTOMATIC DOOR OPERATOR
                                                
                                                        36 - AWNINGS &amp; CANOPIES
                                                
                                                        38 - BEVERAGE DISPENSER ELECTRICAL
                                                
                                                        40 - BEVERAGE DISPENSER PLUMBING
                                                
                                                        42 - BREAK ROOM ELECTRICAL
                                                
                                                        44 - BREAK ROOM PLUMBING
                                                
                                                        46 - BUILDING SIGNAGE
                                                
                                                        48 - BUILDING SIGNAGE ELECTRICAL
                                                
                                                        50 - COFFEE BAR ELECTRICAL
                                                
                                                        52 - COFFEE BAR PLUMBING
                                                
                                                        54 - COMPRESSED AIR SYSTEM ELECTRICAL
                                                
                                                        56 - COMPUTER CONNECTIONS
                                                
                                                        58 - COPIER ELECTRICAL
                                                
                                                        60 - CURBING &amp; BUMPERS, SITE
                                                
                                                        61 - DEMOUNTABLE HVAC SYSTEMS
                                                
                                                        61.5 - DEMOUNTABLE MEZZANINE
                                                
                                                        62 - DOCK EQUIPMENT
                                                
                                                        64 - DOOR, INTERIOR SPECIALTY
                                                
                                                        66 - ELECTRICAL, EQUIPMENT
                                                
                                                        67 - EMERGENCY GENERATOR SYSTEM
                                                
                                                        68 - EMERGENCY EYE WASH
                                                
                                                        70 - FENCING, INTERIOR
                                                
                                                        72 - FENCING, SITE
                                                
                                                        74 - FIRE EXTINGUISHERS
                                                
                                                        76 - FITNESS EQUIPMENT ELECTRICAL
                                                
                                                        78 - FLAGPOLES, SITE
                                                
                                                        80 - FLOORING, REMOVABLE
                                                
                                                        82 - FURNISHINGS, SITE
                                                
                                                        84 - FURNITURE, FIXTURES, &amp; EQUIPMENT
                                                
                                                        86 - GARAGE DOOR OPERATOR
                                                
                                                        88 - GATE OPERATOR
                                                
                                                        90 - GLUED-ON FINISHES
                                                
                                                        91 - INDOOR SWIMMING POOL EQUIPMENT
                                                
                                                        92 - KITCHEN EQUIPMENT
                                                
                                                        94 - KITCHEN ELECTRICAL
                                                
                                                        96 - KITCHEN PLUMBING
                                                
                                                        98 - LANDSCAPING &amp; IRRIGATION
                                                
                                                        100 - LAUNDRY APPLIANCES
                                                
                                                        102 - LAUNDRY ELECTRICAL
                                                
                                                        104 - LAUNDRY PLUMBING
                                                
                                                        106 - LIGHTING, DECORATIVE INTERIOR
                                                
                                                        108 - LIGHTING, DECORATIVE SITE
                                                
                                                        110 - LIGHTING, EXIT &amp; EMERGENCY
                                                
                                                        112 - LIGHTING, EXTERIOR DECORATIVE
                                                
                                                        114 - LIGHTING, PARKING LOT
                                                
                                                        116 - LOADING DOCK, SITE
                                                
                                                        118 - LOCKERS
                                                
                                                        120 - MAILBOXES
                                                
                                                        122 - MILLWORK
                                                
                                                        123 - MOVABLE PARTITIONS
                                                
                                                        124 - OIL SYSTEM PLUMBING
                                                
                                                        125 - PARKING TICKETING EQUIPMENT
                                                
                                                        126 - PARTITIONS, SPECIALTY
                                                
                                                        128 - PARTS DEPARTMENT ELECTRICAL
                                                
                                                        130 - PAVING, WALKS &amp; ROADS
                                                
                                                        132 - PHOTOVOLTAIC SYSTEM
                                                
                                                        134 - PIPE BOLLARDS, INTERIOR
                                                
                                                        136 - PIPE BOLLARDS, SITE
                                                
                                                        138 - PLUMBING, EQUIPMENT
                                                
                                                        140 - REFRIGERATION ELECTRIC
                                                
                                                        142 - REFRIGERATION EQUIPMENT
                                                
                                                        145 - SECURITY SYSTEMS
                                                
                                                        146 - SERVER ROOM HVAC
                                                
                                                        148 - SHOP CRANE
                                                
                                                        150 - SHOP ELECTRICAL
                                                
                                                        152 - SHOP EQUIPMENT STRUCTURE
                                                
                                                        154 - SHOP EXHAUST SYSTEM
                                                
                                                        156 - SHOP PLUMBING
                                                
                                                        158 - SIGNAGE, SITE
                                                
                                                        159 - SITE ATHLETICS COURTS
                                                
                                                        160 - SITE FURNISHING GAS CONNECTIONS
                                                
                                                        161 - SITE PORTE COCHERE
                                                
                                                        162 - SITE PRIVACY WALLS
                                                
                                                        163 - SITE RETAINING WALLS
                                                
                                                        164 - SITE SIGNAGE ELECTRICAL
                                                
                                                        166 - SITE SWIMMING POOLS
                                                
                                                        166.5 - SPECIALTY CONVEYING SYSTEMS
                                                
                                                        167 - SPECIALTY FIRE SUPPRESSION
                                                
                                                        168 - STORM DRAIN SYSTEM
                                                
                                                        170 - SURVEILLANCE SYSTEM
                                                
                                                        172 - SURVEILLANCE SYSTEM CONNECTIONS
                                                
                                                        174 - TELEPHONE SYSTEM
                                                
                                                        176 - TELEPHONE SYSTEM CONNECTIONS
                                                
                                                        178 - TELEVISION / CABLE CONNECTIONS
                                                
                                                        180 - TRASH ENCLOSURE
                                                
                                                        182 - VENDING EQUIPMENT ELECTRICAL
                                                
                                                        184 - WALL PROTECTION / CORNER GUARDS
                                                
                                                        186 - WASH BAY ELECTRICAL
                                                
                                                        188 - WASH BAY PLUMBING
                                                
                                                        190 - WINDOW COVERINGS
                                                
                                                
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_purpose_to_recovery_period_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select-recovery-period').each(function(){
            if(isNaN($(this).val())){
                $(this).siblings(&quot;.recovery_period_error&quot;).text(&quot;Please Select a value&quot;);
                valid = false;
            }
        });

        if (valid){
            $(&quot;#update_purpose_to_recovery_period&quot;).submit();
        }
    });

    
    
        
            
                ×
                Map Asset Classes to Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Map Asset Classes
                                            
                                        
                                        
                                            
                                                Update Recovery Period
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Asset Classes
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select Recovery Period
                                                
                                                    
                                                        5 Years
                                                    
                                                
                                                    
                                                        15 Years
                                                    
                                                
                                                    
                                                        39 Years
                                                    
                                                
                                                    
                                                
                                                Indirect Costs
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_asset_class_to_recovery_period_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select_map_class').each(function(){
            if(!$(this).val()){
                $(this).siblings(&quot;.recovery_period_error&quot;).text(&quot;Please Select a value&quot;);
                valid = false;
            }
        });

        if (valid){
            $(&quot;#update_asset_class_to_recovery_period&quot;).submit();
        }
    });

    
    
        
            
                ×
                Bulk Modify
            
            
                
                    
                        
                            
                                
                                    
                                        
                                            Operations to Perform on Selected Lines: 
                                        
                                        
                                            
                                                Clone
                                                Delete
                                                Delete Zero Quantities
                                                Modify Asset Class
                                                Modify Cost Sources
                                                Modify Cost Units
                                                Modify Equipment Cost Factors
                                                Modify Global Pricing Factors (Depreciation / Appreciation)
                                                Modify Labor Cost Factors
                                                Modify Locations
                                                Modify Material Cost Factors
                                                Modify Purpose Description Prefixes
                                                Modify Recovery Period
                                                Modify Takeoff Cost Associations
                                                Modify Takeoff Cost Designations
                                                Modify Quantities
                                                Multiply Locations
                                                Replace Selected Keywords
                                                Revert All Cost Factors to 1.0
                                                Revert Costs to Original Database Values
                                                Zero Out Quantities
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Select
                    Cancel
                
            
        
    


    function open_bulk_tool(){
        tool_selected = $(&quot;#id_select_bulk_tool&quot;).val();
        $('#bulk_tool_select').modal('hide');
        $(&quot;#&quot;+tool_selected).modal(&quot;show&quot;);
        if(tool_selected == 'replace_keywords'){
            search = $(&quot;#bulk-selection-form&quot;).data(&quot;search&quot;);
            $(&quot;#replace_keywords #id_original_keyword&quot;).val(search);
        }
        $(&quot;#bulk-selection-form&quot;).data(&quot;search&quot;, &quot;&quot;)
    }
    $(&quot;#bulk-selection-form-button&quot;).click(function(e){
        e.preventDefault();
        open_bulk_tool();
    });

    $('#id_select_bulk_tool>option').hover(function(e){
        if($(this).is(&quot;:hover&quot;)){
            $(this).css({&quot;background-color&quot;: &quot;rgb(7, 139, 234)&quot;, &quot;color&quot;:&quot;white&quot;, &quot;cursor&quot;: &quot;pointer&quot;});
        }else{
            $(this).css({&quot;background-color&quot;: &quot;white&quot;, &quot;color&quot;: &quot;#111&quot;});
        }
    });


    
    
        
            
                ×
                Bulk Modify
            
            
                
                    
                        
                            
                                
                                    
                                        
                                            Operations to Perform on Selected Lines: 
                                        
                                        
                                            
                                                Clone
                                                Delete
                                                Delete Zero Quantities
                                                Modify Asset Class
                                                Modify Cost Sources
                                                Modify Cost Units
                                                Modify Equipment Cost Factors
                                                Modify Global Pricing Factors (Depreciation / Appreciation)
                                                Modify Labor Cost Factors
                                                Modify Locations
                                                Modify Material Cost Factors
                                                Modify Purpose Description Prefixes
                                                Modify Recovery Period
                                                Modify Takeoff Cost Associations
                                                Modify Takeoff Cost Designations
                                                Modify Quantities
                                                Multiply Locations
                                                Replace Selected Keywords
                                                Revert All Cost Factors to 1.0
                                                Revert Costs to Original Database Values
                                                Zero Out Quantities
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Select
                    Cancel
                
            
        
    


    function open_bulk_filter_tool(){
        tool_selected = $(&quot;#id_select_bulk_filter_tool&quot;).val();
        $('#bulk_tool_select_filter').modal('hide');
        $(&quot;#&quot;+tool_selected).modal(&quot;show&quot;);
        if(tool_selected == 'replace_keywords'){
            search = $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;);
            $(&quot;#replace_keywords #id_original_keyword&quot;).val(search);
        }
        $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;, &quot;&quot;)
    }

    function open_bulk_filter_modify_tool(){
        // tool_selected = $(&quot;#id_select_bulk_filter_tool&quot;).val();
        window.localStorage.setItem(&quot;open_filter_tool&quot;, &quot;true&quot;)
        queryFilteredItems()
        // $('#itemFilter').modal('hide');
        // $('#bulk_tool_select_filter').modal('show');
        // $(&quot;#&quot;+tool_selected).modal(&quot;show&quot;);
        // if(tool_selected == 'replace_keywords'){
        //     search = $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;);
        //     $(&quot;#replace_keywords #id_original_keyword&quot;).val(search);
        // }
        // $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;, &quot;&quot;)
    }

    $(&quot;#bulk-filter-form-button&quot;).click(function(e){
        e.preventDefault();
        open_bulk_filter_tool();
    });

    $('#id_select_bulk_filter_tool>option').hover(function(e){
        if($(this).is(&quot;:hover&quot;)){
            $(this).css({&quot;background-color&quot;: &quot;rgb(7, 139, 234)&quot;, &quot;color&quot;:&quot;white&quot;, &quot;cursor&quot;: &quot;pointer&quot;});
        }else{
            $(this).css({&quot;background-color&quot;: &quot;white&quot;, &quot;color&quot;: &quot;#111&quot;});
        }
    });


    
    
        
            
                ×
                Update multiple Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Recovery Period of  selected records.
                                        
                                        
                                            Recovery Period: 
                                        
                                        
                                            
                                                
                                                    5 Years
                                                
                                                    15 Years
                                                
                                                    39 Years
                                                
                                                    Indirect Costs
                                                
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Recovery Period of  selected records.
                                        
                                        
                                            Recovery Period: 
                                        
                                        
                                            
                                                
                                                    5 Years
                                                
                                                    15 Years
                                                
                                                    39 Years
                                                
                                                    Indirect Costs
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update Location Multiplier
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Location Multiplier of  selected records.
                                        
                                        
                                            Location: 
                                        
                                        
                                            Location Multiplier: 
                                        
                                        
                                        
                                        
                                    
                                    
                                        
                                           
                                            Select
                                            
                                           
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $('#location_filter_list').on('change', function(e){
        var location_modifier = $(this).val();
        var building_id = $('option:selected', this).attr('data-building_id');
        var location = $('option:selected', this).attr('data-location');

        $('#filter_building_location').val(building_id);
        $('#filter_location_id').val(location);
        $('#filter_location_modifier').prop('disabled', false).val(location_modifier);

    })
    $(&quot;#change_filter_location_modifier_btn&quot;).on(&quot;click&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $(&quot;.location_modifier&quot;).each(function(){
            $(this).closest(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);


            if(!$(this).val()){
                $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a value&quot;)
                $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);

                valid=false
            }
            else{
                if(isNaN(parseFloat($(this).val()))){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
                else if(parseFloat($(&quot;.location_modifier&quot;).val()) &lt;= 0){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
            }

        })
        if (valid){
            $(&quot;#filter_location_modifier_form&quot;).submit();
        }
    })

    function location_modifier_clear_validation(){
        $(&quot;.location_modifier_error&quot;).text(&quot;&quot;)
        $(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);
    }


    
    
        
            
                ×
                Update multiple Asset Class
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Asset class of  selected records.
                                        
                                        
                                            Asset Class: 
                                        
                                        
                                            
                                                Select
                                                
                                                    1 - GENERAL REQUIREMENTS
                                                
                                                    2 - SITEWORK
                                                
                                                    3 - CONCRETE
                                                
                                                    4 - MASONRY
                                                
                                                    5 - METALS
                                                
                                                    6 - WOOD &amp; PLASTICS
                                                
                                                    7 - THERMAL &amp; MOISTURE PROTECTION
                                                
                                                    8 - DOORS &amp; WINDOWS
                                                
                                                    9 - FINISHES
                                                
                                                    10 - SPECIALTIES
                                                
                                                    11 - EQUIPMENT
                                                
                                                    12 - FURNISHINGS
                                                
                                                    13 - SPECIAL CONSTRUCTION
                                                
                                                    14 - CONVEYING SYSTEMS
                                                
                                                    15 - PLUMBING
                                                
                                                    16 - MECHANICAL
                                                
                                                    17 - HVAC
                                                
                                                    18 - ELECTRICAL
                                                
                                                    19 - BUILDING GAS SYSTEM
                                                
                                                    20 - BUILDING FIRE PROTECTION
                                                
                                                    21 - SITE FIRE PROTECTION
                                                
                                                    30 - ATM ELECTRICAL
                                                
                                                    32 - AUDIO/VISUAL SYSTEM
                                                
                                                    34 - AUTOMATIC DOOR OPERATOR
                                                
                                                    36 - AWNINGS &amp; CANOPIES
                                                
                                                    38 - BEVERAGE DISPENSER ELECTRICAL
                                                
                                                    40 - BEVERAGE DISPENSER PLUMBING
                                                
                                                    42 - BREAK ROOM ELECTRICAL
                                                
                                                    44 - BREAK ROOM PLUMBING
                                                
                                                    46 - BUILDING SIGNAGE
                                                
                                                    48 - BUILDING SIGNAGE ELECTRICAL
                                                
                                                    50 - COFFEE BAR ELECTRICAL
                                                
                                                    52 - COFFEE BAR PLUMBING
                                                
                                                    54 - COMPRESSED AIR SYSTEM ELECTRICAL
                                                
                                                    56 - COMPUTER CONNECTIONS
                                                
                                                    58 - COPIER ELECTRICAL
                                                
                                                    60 - CURBING &amp; BUMPERS, SITE
                                                
                                                    61 - DEMOUNTABLE HVAC SYSTEMS
                                                
                                                    61.5 - DEMOUNTABLE MEZZANINE
                                                
                                                    62 - DOCK EQUIPMENT
                                                
                                                    64 - DOOR, INTERIOR SPECIALTY
                                                
                                                    66 - ELECTRICAL, EQUIPMENT
                                                
                                                    67 - EMERGENCY GENERATOR SYSTEM
                                                
                                                    68 - EMERGENCY EYE WASH
                                                
                                                    70 - FENCING, INTERIOR
                                                
                                                    72 - FENCING, SITE
                                                
                                                    74 - FIRE EXTINGUISHERS
                                                
                                                    76 - FITNESS EQUIPMENT ELECTRICAL
                                                
                                                    78 - FLAGPOLES, SITE
                                                
                                                    80 - FLOORING, REMOVABLE
                                                
                                                    82 - FURNISHINGS, SITE
                                                
                                                    84 - FURNITURE, FIXTURES, &amp; EQUIPMENT
                                                
                                                    86 - GARAGE DOOR OPERATOR
                                                
                                                    88 - GATE OPERATOR
                                                
                                                    90 - GLUED-ON FINISHES
                                                
                                                    91 - INDOOR SWIMMING POOL EQUIPMENT
                                                
                                                    92 - KITCHEN EQUIPMENT
                                                
                                                    94 - KITCHEN ELECTRICAL
                                                
                                                    96 - KITCHEN PLUMBING
                                                
                                                    98 - LANDSCAPING &amp; IRRIGATION
                                                
                                                    100 - LAUNDRY APPLIANCES
                                                
                                                    102 - LAUNDRY ELECTRICAL
                                                
                                                    104 - LAUNDRY PLUMBING
                                                
                                                    106 - LIGHTING, DECORATIVE INTERIOR
                                                
                                                    108 - LIGHTING, DECORATIVE SITE
                                                
                                                    110 - LIGHTING, EXIT &amp; EMERGENCY
                                                
                                                    112 - LIGHTING, EXTERIOR DECORATIVE
                                                
                                                    114 - LIGHTING, PARKING LOT
                                                
                                                    116 - LOADING DOCK, SITE
                                                
                                                    118 - LOCKERS
                                                
                                                    120 - MAILBOXES
                                                
                                                    122 - MILLWORK
                                                
                                                    123 - MOVABLE PARTITIONS
                                                
                                                    124 - OIL SYSTEM PLUMBING
                                                
                                                    125 - PARKING TICKETING EQUIPMENT
                                                
                                                    126 - PARTITIONS, SPECIALTY
                                                
                                                    128 - PARTS DEPARTMENT ELECTRICAL
                                                
                                                    130 - PAVING, WALKS &amp; ROADS
                                                
                                                    132 - PHOTOVOLTAIC SYSTEM
                                                
                                                    134 - PIPE BOLLARDS, INTERIOR
                                                
                                                    136 - PIPE BOLLARDS, SITE
                                                
                                                    138 - PLUMBING, EQUIPMENT
                                                
                                                    140 - REFRIGERATION ELECTRIC
                                                
                                                    142 - REFRIGERATION EQUIPMENT
                                                
                                                    145 - SECURITY SYSTEMS
                                                
                                                    146 - SERVER ROOM HVAC
                                                
                                                    148 - SHOP CRANE
                                                
                                                    150 - SHOP ELECTRICAL
                                                
                                                    152 - SHOP EQUIPMENT STRUCTURE
                                                
                                                    154 - SHOP EXHAUST SYSTEM
                                                
                                                    156 - SHOP PLUMBING
                                                
                                                    158 - SIGNAGE, SITE
                                                
                                                    159 - SITE ATHLETICS COURTS
                                                
                                                    160 - SITE FURNISHING GAS CONNECTIONS
                                                
                                                    161 - SITE PORTE COCHERE
                                                
                                                    162 - SITE PRIVACY WALLS
                                                
                                                    163 - SITE RETAINING WALLS
                                                
                                                    164 - SITE SIGNAGE ELECTRICAL
                                                
                                                    166 - SITE SWIMMING POOLS
                                                
                                                    166.5 - SPECIALTY CONVEYING SYSTEMS
                                                
                                                    167 - SPECIALTY FIRE SUPPRESSION
                                                
                                                    168 - STORM DRAIN SYSTEM
                                                
                                                    170 - SURVEILLANCE SYSTEM
                                                
                                                    172 - SURVEILLANCE SYSTEM CONNECTIONS
                                                
                                                    174 - TELEPHONE SYSTEM
                                                
                                                    176 - TELEPHONE SYSTEM CONNECTIONS
                                                
                                                    178 - TELEVISION / CABLE CONNECTIONS
                                                
                                                    180 - TRASH ENCLOSURE
                                                
                                                    182 - VENDING EQUIPMENT ELECTRICAL
                                                
                                                    184 - WALL PROTECTION / CORNER GUARDS
                                                
                                                    186 - WASH BAY ELECTRICAL
                                                
                                                    188 - WASH BAY PLUMBING
                                                
                                                    190 - WINDOW COVERINGS
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                
                
                    ×
                     Import Costs Setup
                
                
                    
                        
                            Upload File
                        
                        
                            
                        
                    
                
            
            
                Upload
                Cancel
            
        
    


    
    
        
            
                ×
                Update Material Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Material Cost of  selected records.
                                        
                                        
                                            Material Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Material Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Material Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Equipment Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Equipment Cost of  selected records.
                                        
                                        
                                            Equipment Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Equipment Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Equipment Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Replace Keyword
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Replace Keyword of  selected records.
                                        
                                        
                                            Keyword: 
                                        
                                        
                                            
                                        
                                    
                                    
                                        
                                            Replace By: 
                                        
                                        
                                            
                                        
                                    
                                    
                                    
                                
                            
                        
                    
                    Note: This tool will not search or modify text located inside of Purpose Description Prefixes.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Replace Keyword
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Keyword: 
                                        
                                        
                                            
                                        
                                    
                                    
                                        
                                            Replace By: 
                                        
                                        
                                            
                                        
                                    
                                    
                                    
                                    
                                
                            
                        
                    
                    Note: This tool will not search or modify text located inside of Purpose Description Prefixes.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Labour Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Labour Cost Factor of  selected records.
                                        
                                        
                                            Labour Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
            
            
                Update
                Update &amp; Modify Same Items Again
                Cancel
            
        
    



    $(document).ready(function(){
        $(document).on('click', '.button_submit', function(e){
            e.preventDefault();
            var valid = true;
            if (!$('#id_updated_labour_cost').val() || !($('#id_updated_labour_cost').val().length &lt; 8)){
                var valid = false

                $('#labour_cost_invalid').text('Invalid input')
            }
            if (valid){
                $('#labour_cost_update_form').submit();
            }
        })
    });


    
    
        
            
                ×
                Update Labour Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Labour Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
            
            
                Update
                Update &amp; Modify Same Items Again
                Cancel
            
        
    



    $(document).ready(function(){
        $(document).on('click', '.button_filter_submit', function(e){
            e.preventDefault();
            var valid = true;
            if (!$('#id_filter_updated_labour_cost').val() || !($('#id_filter_updated_labour_cost').val().length &lt; 8)){
                var valid = false

                $('#labour_cost_invalid_filter').text('Invalid input')
            }
            if (valid){
                $('#labour_cost_filter_update_form').submit();
            }
        })
    });


    
    
        
            
                ×
                Designate/Undesignate as Takeoff Cost
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Designate/Undesignate  selected records as Takeoff Cost.
                                        
                                        
                                            Takeoff Cost: 
                                        
                                        
                                            
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Only Custom Items can be designated as Takeoff Costs.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Designate/Undesignate as Takeoff Cost
            
            
                
                
                    
                        
                            
                                
                                    

                                        
                                            Takeoff Cost: 
                                        
                                        
                                            
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Only Custom Items can be designated as Takeoff Costs.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Cost Source
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Cost Source of  selected records.
                                        
                                    
                                    
                                        
                                            Cost Source: 
                                        
                                        
                                            
                                                    Select
                                                
                                                    Contractor Cost
                                                
                                                    Client Cost
                                                
                                                    RS Means
                                                
                                                    Site Contractor Cost
                                                
                                                    Site Indirect
                                                
                                                    Client Cost - Expenses
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple Cost Source
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Cost Source: 
                                        
                                        
                                            
                                                    Select
                                                
                                                    Contractor Cost
                                                
                                                    Client Cost
                                                
                                                    RS Means
                                                
                                                    Site Contractor Cost
                                                
                                                    Site Indirect
                                                
                                                    Client Cost - Expenses
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple unit
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Cost Unit of  selected records.
                                        
                                        
                                            Unit: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple unit
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Cost Unit of  selected records.
                                        
                                        
                                            Unit: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Asset Class
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Asset class of  selected records.
                                        
                                        
                                            Asset Class: 
                                        
                                        
                                            
                                                Select
                                                
                                                    1 - GENERAL REQUIREMENTS
                                                
                                                    2 - SITEWORK
                                                
                                                    3 - CONCRETE
                                                
                                                    4 - MASONRY
                                                
                                                    5 - METALS
                                                
                                                    6 - WOOD &amp; PLASTICS
                                                
                                                    7 - THERMAL &amp; MOISTURE PROTECTION
                                                
                                                    8 - DOORS &amp; WINDOWS
                                                
                                                    9 - FINISHES
                                                
                                                    10 - SPECIALTIES
                                                
                                                    11 - EQUIPMENT
                                                
                                                    12 - FURNISHINGS
                                                
                                                    13 - SPECIAL CONSTRUCTION
                                                
                                                    14 - CONVEYING SYSTEMS
                                                
                                                    15 - PLUMBING
                                                
                                                    16 - MECHANICAL
                                                
                                                    17 - HVAC
                                                
                                                    18 - ELECTRICAL
                                                
                                                    19 - BUILDING GAS SYSTEM
                                                
                                                    20 - BUILDING FIRE PROTECTION
                                                
                                                    21 - SITE FIRE PROTECTION
                                                
                                                    30 - ATM ELECTRICAL
                                                
                                                    32 - AUDIO/VISUAL SYSTEM
                                                
                                                    34 - AUTOMATIC DOOR OPERATOR
                                                
                                                    36 - AWNINGS &amp; CANOPIES
                                                
                                                    38 - BEVERAGE DISPENSER ELECTRICAL
                                                
                                                    40 - BEVERAGE DISPENSER PLUMBING
                                                
                                                    42 - BREAK ROOM ELECTRICAL
                                                
                                                    44 - BREAK ROOM PLUMBING
                                                
                                                    46 - BUILDING SIGNAGE
                                                
                                                    48 - BUILDING SIGNAGE ELECTRICAL
                                                
                                                    50 - COFFEE BAR ELECTRICAL
                                                
                                                    52 - COFFEE BAR PLUMBING
                                                
                                                    54 - COMPRESSED AIR SYSTEM ELECTRICAL
                                                
                                                    56 - COMPUTER CONNECTIONS
                                                
                                                    58 - COPIER ELECTRICAL
                                                
                                                    60 - CURBING &amp; BUMPERS, SITE
                                                
                                                    61 - DEMOUNTABLE HVAC SYSTEMS
                                                
                                                    61.5 - DEMOUNTABLE MEZZANINE
                                                
                                                    62 - DOCK EQUIPMENT
                                                
                                                    64 - DOOR, INTERIOR SPECIALTY
                                                
                                                    66 - ELECTRICAL, EQUIPMENT
                                                
                                                    67 - EMERGENCY GENERATOR SYSTEM
                                                
                                                    68 - EMERGENCY EYE WASH
                                                
                                                    70 - FENCING, INTERIOR
                                                
                                                    72 - FENCING, SITE
                                                
                                                    74 - FIRE EXTINGUISHERS
                                                
                                                    76 - FITNESS EQUIPMENT ELECTRICAL
                                                
                                                    78 - FLAGPOLES, SITE
                                                
                                                    80 - FLOORING, REMOVABLE
                                                
                                                    82 - FURNISHINGS, SITE
                                                
                                                    84 - FURNITURE, FIXTURES, &amp; EQUIPMENT
                                                
                                                    86 - GARAGE DOOR OPERATOR
                                                
                                                    88 - GATE OPERATOR
                                                
                                                    90 - GLUED-ON FINISHES
                                                
                                                    91 - INDOOR SWIMMING POOL EQUIPMENT
                                                
                                                    92 - KITCHEN EQUIPMENT
                                                
                                                    94 - KITCHEN ELECTRICAL
                                                
                                                    96 - KITCHEN PLUMBING
                                                
                                                    98 - LANDSCAPING &amp; IRRIGATION
                                                
                                                    100 - LAUNDRY APPLIANCES
                                                
                                                    102 - LAUNDRY ELECTRICAL
                                                
                                                    104 - LAUNDRY PLUMBING
                                                
                                                    106 - LIGHTING, DECORATIVE INTERIOR
                                                
                                                    108 - LIGHTING, DECORATIVE SITE
                                                
                                                    110 - LIGHTING, EXIT &amp; EMERGENCY
                                                
                                                    112 - LIGHTING, EXTERIOR DECORATIVE
                                                
                                                    114 - LIGHTING, PARKING LOT
                                                
                                                    116 - LOADING DOCK, SITE
                                                
                                                    118 - LOCKERS
                                                
                                                    120 - MAILBOXES
                                                
                                                    122 - MILLWORK
                                                
                                                    123 - MOVABLE PARTITIONS
                                                
                                                    124 - OIL SYSTEM PLUMBING
                                                
                                                    125 - PARKING TICKETING EQUIPMENT
                                                
                                                    126 - PARTITIONS, SPECIALTY
                                                
                                                    128 - PARTS DEPARTMENT ELECTRICAL
                                                
                                                    130 - PAVING, WALKS &amp; ROADS
                                                
                                                    132 - PHOTOVOLTAIC SYSTEM
                                                
                                                    134 - PIPE BOLLARDS, INTERIOR
                                                
                                                    136 - PIPE BOLLARDS, SITE
                                                
                                                    138 - PLUMBING, EQUIPMENT
                                                
                                                    140 - REFRIGERATION ELECTRIC
                                                
                                                    142 - REFRIGERATION EQUIPMENT
                                                
                                                    145 - SECURITY SYSTEMS
                                                
                                                    146 - SERVER ROOM HVAC
                                                
                                                    148 - SHOP CRANE
                                                
                                                    150 - SHOP ELECTRICAL
                                                
                                                    152 - SHOP EQUIPMENT STRUCTURE
                                                
                                                    154 - SHOP EXHAUST SYSTEM
                                                
                                                    156 - SHOP PLUMBING
                                                
                                                    158 - SIGNAGE, SITE
                                                
                                                    159 - SITE ATHLETICS COURTS
                                                
                                                    160 - SITE FURNISHING GAS CONNECTIONS
                                                
                                                    161 - SITE PORTE COCHERE
                                                
                                                    162 - SITE PRIVACY WALLS
                                                
                                                    163 - SITE RETAINING WALLS
                                                
                                                    164 - SITE SIGNAGE ELECTRICAL
                                                
                                                    166 - SITE SWIMMING POOLS
                                                
                                                    166.5 - SPECIALTY CONVEYING SYSTEMS
                                                
                                                    167 - SPECIALTY FIRE SUPPRESSION
                                                
                                                    168 - STORM DRAIN SYSTEM
                                                
                                                    170 - SURVEILLANCE SYSTEM
                                                
                                                    172 - SURVEILLANCE SYSTEM CONNECTIONS
                                                
                                                    174 - TELEPHONE SYSTEM
                                                
                                                    176 - TELEPHONE SYSTEM CONNECTIONS
                                                
                                                    178 - TELEVISION / CABLE CONNECTIONS
                                                
                                                    180 - TRASH ENCLOSURE
                                                
                                                    182 - VENDING EQUIPMENT ELECTRICAL
                                                
                                                    184 - WALL PROTECTION / CORNER GUARDS
                                                
                                                    186 - WASH BAY ELECTRICAL
                                                
                                                    188 - WASH BAY PLUMBING
                                                
                                                    190 - WINDOW COVERINGS
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Location
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Location: 
                                        
                                        
                                            
                                                Select
                                            
                                                Building
                                            
                                                Building Interior
                                            
                                                Site Exterior
                                            
                                                Add Additional Location
                                            
                                            
                                            
                                            
                                        
                                    
                                    
                                        
                                            Building
                                        
                                        
                                            
                                                
                                            
                                            
                                        
                                    
                                    
                                        
                                            Extra Location
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Clone
            
            
                
                
                    
                            
                                
                                    
                                        
                                            
                                                Clone  selected records.
                                            
                                            
                                                Number of Clones to Add: 
                                            
                                            
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                
                            
                        
                        
                            Note:
                                
                                    Cloned Items will be added as new components and will not appear alongside the original component when Viewed by Input Order (or Reverse Input Order).
                                
                            
                        
                
                
                    Clone
                    Clone &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Clone
            
            
                
                
                    
                            
                                
                                    
                                        
                                            
                                                Number of Clones to Add: 
                                            
                                            
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                
                            
                        
                        
                            Note:
                                
                                    Cloned Items will be added as new components and will not appear alongside the original component when Viewed by Input Order (or Reverse Input Order).
                                
                            
                        
                
                
                    Clone
                    Clone &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Takeoff Cost Association
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Takeoff Cost Association of  selected records.
                                        
                                        
                                            Select Takeoff Cost: 
                                        
                                        
                                            
                                                None
                                                
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Takeoff Cost Association
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Select Takeoff Cost: 
                                        
                                        
                                            
                                                None
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                 Change Password
            
            
                
                
                    
                        
                            
                                Old Password
                            
                            
                                
                                
                            
                        
                        
                            
                                New Password
                            
                            
                                
                                
                            
                        
                        
                            
                                Confirm Password
                            
                            
                                
                                
                            
                        
                    
                
                
                    Save
                    Close
                
            
        
    


    
    
        
                  Please Select atleast one Item
            ×
        
    



    
        
                  No line with zero quantity
            ×
        
    


    

    
        
            
                ×
                 Add Takeoff or Costs
            
            
                
                     Add Normal Items/Assemblies
                     Add Costs/Custom Items
                    
                     Add Custom Assembly Wizard
                
                
                    
                        
                            
                                
                                    
                                        
                                        
                                            
                                                
                                                    
                                                        
                                                            
                                                            Add Items
                                                        
                                                        
                                                            
                                                            Add Assemblies
                                                        
                                                        
                                                            
                                                            Both
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            
                                                            Normal Search
                                                        
                                                        
                                                            
                                                            Favorites Filter
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    
                                                        
                                                            Construction Division Filter
                                                        
                                                        
                                                            Select
                                                            0. Custom Item
                                                            
                                                                1. General Requirements 
                                                            
                                                                2. Existing Conditions 
                                                            
                                                                3. Concrete 
                                                            
                                                                4. Masonry 
                                                            
                                                                5. Metals 
                                                            
                                                                6. Wood, Plastics, and Composites 
                                                            
                                                                7. Thermal and Moisture Protection 
                                                            
                                                                8. Openings 
                                                            
                                                                9. Finishes 
                                                            
                                                                10. Specialties 
                                                            
                                                                11. Equipment 
                                                            
                                                                12. Furnishings 
                                                            
                                                                13. Special Construction 
                                                            
                                                                14. Conveying Equipment 
                                                            
                                                                21. Fire Suppression 
                                                            
                                                                22. Plumbing 
                                                            
                                                                23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                                            
                                                                26. Electrical 
                                                            
                                                                27. Communications 
                                                            
                                                                28. Electronic Safety and Security 
                                                            
                                                                31. Earthwork 
                                                            
                                                                32. Exterior Improvements 
                                                            
                                                                33. Utilities 
                                                            
                                                                34. Transportation 
                                                            
                                                                35. Waterway and Marine Construction 
                                                            
                                                                41. Material Processing and Handling Equipment 
                                                            
                                                                44. Pollution and Waste Control Equipment 
                                                            
                                                                50. Landscaping Equipent 
                                                            
                                                                46. Water and Wastewater Equipment 
                                                            
                                                                48. Electrical Power Generation 
                                                            
                                                        
                                                    
                                                    

                                                    
                                                
                                                
                                                    
                                                        Assembly Group Filter
                                                        
                                                            Select
                                                            0. Custom Assembly
                                                            
                                                                1. Standard Foundations
                                                            
                                                                2. Special Foundations
                                                            
                                                                3. Slab on Grade
                                                            
                                                                4. Basement Excavation
                                                            
                                                                5. Basement Walls
                                                            
                                                                6. Floor Construction
                                                            
                                                                7. Roof Construction
                                                            
                                                                8. Exterior Walls
                                                            
                                                                9. Exterior Windows
                                                            
                                                                10. Exterior Doors
                                                            
                                                                11. Roof Coverings
                                                            
                                                                12. Roof Openings
                                                            
                                                                13. Partitions
                                                            
                                                                14. Interior Doors
                                                            
                                                                15. Fittings
                                                            
                                                                16. Stair Construction
                                                            
                                                                17. Wall Finishes
                                                            
                                                                18. Floor Finishes
                                                            
                                                                19. Ceiling Finishes
                                                            
                                                                20. Elevators and Lifts
                                                            
                                                                21. Escalators and Moving Walks
                                                            
                                                                22. Other Conveying Systems
                                                            
                                                                23. Plumbing Fixtures
                                                            
                                                                24. Domestic Water Distribution
                                                            
                                                                25. Rain Water Drainage
                                                            
                                                                26. Other Plumbing Systems
                                                            
                                                                27. Energy Supply
                                                            
                                                                28. Heat Generating Systems
                                                            
                                                                29. Cooling Generating Systems
                                                            
                                                                30. Terminal &amp; Package Units
                                                            
                                                                31. Sprinklers
                                                            
                                                                32. Standpipes
                                                            
                                                                33. Other Fire Protection Systems
                                                            
                                                                34. Electrical Service/Distribution
                                                            
                                                                35. Lighting and Branch Wiring
                                                            
                                                                36. Communications and Security
                                                            
                                                                37. Other Electrical Systems
                                                            
                                                                38. Commercial Equipment
                                                            
                                                                39. Institutional Equipment
                                                            
                                                                40. Vehicular Equipment
                                                            
                                                                41. Other Equipment
                                                            
                                                                42. Fixed Furnishings
                                                            
                                                                43. Moveable Furnishings
                                                            
                                                                44. Special Structures
                                                            
                                                                45. Integrated Construction
                                                            
                                                                46. Special Construction Systems
                                                            
                                                                47. Special Facilities
                                                            
                                                                48. Site Earthwork
                                                            
                                                                49. Roadways
                                                            
                                                                50. Parking Lots
                                                            
                                                                51. Pedestrian Paving
                                                            
                                                                52. Site Development
                                                            
                                                                53. Water Supply
                                                            
                                                                54. Sanitary Sewer
                                                            
                                                                55. Storm Sewer
                                                            
                                                                56. Fuel Distribution
                                                            
                                                                57. Site Lighting
                                                            
                                                        
                                                    
                                                    

                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    Favorites Item/Assembly
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            Items Search Result
                                                            
                                                            
                                                        
                                                        
                                                            Assemblies Search Result
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                    
                                                    
                                                        View/Modify Assembly
                                                    
                                                
                                            
                                            

                                            

                                            
                                                
                                                    
                                                        
                                                            Quantity
                                                        

                                                        
                                                            
                                                            
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            Recovery Period
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                                    5 Years
                                                                
                                                                    15 Years
                                                                
                                                                    39 Years
                                                                
                                                                    Indirect Costs
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            
                                                                Purpose Description
                                                            
                                                        
                                                        
                                                        
                                                            
                                                                    Select
                                                                
                                                                    A/V System
                                                                
                                                                    A/V System Equipment
                                                                
                                                                    A/V System Equipment Electric
                                                                
                                                                    A/V System,
                                                                
                                                                    Back Porch
                                                                
                                                                    Backsplash Wall
                                                                
                                                                    Balcony Exterior Doors,
                                                                
                                                                    Balcony Floor Finish,
                                                                
                                                                    Balcony Steel Railing,
                                                                
                                                                    Balcony Wall Light,
                                                                
                                                                    Base
                                                                
                                                                    Base Molding,
                                                                
                                                                    Basement Staircase,
                                                                
                                                                    Bldg
                                                                
                                                                    Bldg Basement
                                                                
                                                                    Bldg Basement Concrete Floor,
                                                                
                                                                    Bldg Basement Excavation, 
                                                                
                                                                    Bldg Basement FDN Wall
                                                                
                                                                    Bldg CC Column
                                                                
                                                                    Bldg CC Elevated Floor Structure
                                                                
                                                                    Bldg CC Footing
                                                                
                                                                    Bldg CC Footings
                                                                
                                                                    Bldg CC Slab
                                                                
                                                                    Bldg CC Slab on Grade
                                                                
                                                                    Bldg Column Fireproofing,
                                                                
                                                                    Bldg Concrete Ext Wall
                                                                
                                                                    Bldg Concrete Staircase,
                                                                
                                                                    Bldg Elev Beam FDN
                                                                
                                                                    Bldg Elev Concrete Floor
                                                                
                                                                    Bldg Elev Floor Structure
                                                                
                                                                    Bldg Elev Metal Joist Floor
                                                                
                                                                    Bldg Elev Structural Steel Floor
                                                                
                                                                    Bldg Elev Walkway
                                                                
                                                                    Bldg Elev Wood Joist Floor
                                                                
                                                                    Bldg Elevated Floor Structure
                                                                
                                                                    Bldg Ext Brick Stucco Wall
                                                                
                                                                    Bldg Ext Brick Veneer Wall
                                                                
                                                                    Bldg Ext Brick Wall
                                                                
                                                                    Bldg Ext CMU EIFS Wall
                                                                
                                                                    Bldg Ext CMU Stucco Wall
                                                                
                                                                    Bldg Ext CMU Wall
                                                                
                                                                    Bldg Ext Column Covering
                                                                
                                                                    Bldg Ext Concrete Wall Panels
                                                                
                                                                    Bldg Ext Demountable Canopy
                                                                
                                                                    Bldg Ext EIFS Wall
                                                                
                                                                    Bldg Ext Glass Curtain Wall
                                                                
                                                                    Bldg Ext Lighting
                                                                
                                                                    Bldg Ext Metal Panel Curtain Wall
                                                                
                                                                    Bldg Ext Metal Siding Wall
                                                                
                                                                    Bldg Ext Metal Stud EIFS Wall
                                                                
                                                                    Bldg Ext Metal Stud Fiber Cement Wall
                                                                
                                                                    Bldg Ext Metal Stud Stucco Wall
                                                                
                                                                    Bldg Ext Metal Stud Wall
                                                                
                                                                    Bldg Ext Stone Veneer Wall
                                                                
                                                                    Bldg Ext Tilt-Up CC Panel Wall
                                                                
                                                                    Bldg Ext Vinyl Siding Wall
                                                                
                                                                    Bldg Ext Wall
                                                                
                                                                    Bldg Ext Wall Lighting
                                                                
                                                                    Bldg Ext Wall Lighting Electrical
                                                                
                                                                    Bldg Ext Wood Siding Wall
                                                                
                                                                    Bldg Ext Wood Stud EIFS Wall
                                                                
                                                                    Bldg Ext Wood Stud Fiber Cement Wall
                                                                
                                                                    Bldg Ext Wood Stud Stucco Wall
                                                                
                                                                    Bldg Ext Wood Stud Wall
                                                                
                                                                    Bldg FDN
                                                                
                                                                    Bldg Masonry Ext Wall
                                                                
                                                                    Bldg Metal Ext Wall
                                                                
                                                                    Bldg PEMB Structure
                                                                
                                                                    Bldg Porte Cochere Brick Veneer Wall
                                                                
                                                                    Bldg Porte Cochere Column Covering
                                                                
                                                                    Bldg Porte Cochere Concrete Footing
                                                                
                                                                    Bldg Porte Cochere EIFS Wall
                                                                
                                                                    Bldg Porte Cochere Lighting
                                                                
                                                                    Bldg Porte Cochere Roof Cover
                                                                
                                                                    Bldg Porte Cochere Roof Drainage
                                                                
                                                                    Bldg Porte Cochere Roof Structure
                                                                
                                                                    Bldg Porte Cochere Steel Beams
                                                                
                                                                    Bldg Porte Cochere Steel Column
                                                                
                                                                    Bldg Porte Cochere Stone Veneer Wall
                                                                
                                                                    Bldg Porte Cochere Wood Beams
                                                                
                                                                    Bldg Porte Cochere Wood Columns
                                                                
                                                                    Bldg Porte Cochere Wood Stud Stucco Wall
                                                                
                                                                    Bldg Roof
                                                                
                                                                    Bldg Roof Cover
                                                                
                                                                    Bldg Roof Covering
                                                                
                                                                    Bldg Roof Drainage
                                                                
                                                                    Bldg Roof Structure
                                                                
                                                                    Bldg Staircase,
                                                                
                                                                    Bldg Steel Column
                                                                
                                                                    Bldg Steel Railing,
                                                                
                                                                    Bldg Steel Staircase,
                                                                
                                                                    Bldg Strip Foundation
                                                                
                                                                    Bldg Structural Steel Beam
                                                                
                                                                    Bldg Unit Concrete Staircase,
                                                                
                                                                    Bldg Unit Staircase,
                                                                
                                                                    Bldg Unit Steel Staircase,
                                                                
                                                                    Bldg Unit Wood Staircase,
                                                                
                                                                    Bldg Walkway Concrete Finishing,
                                                                
                                                                    Bldg Walkway Steel Railing,
                                                                
                                                                    Bldg Wd Elevated Floor Structure
                                                                
                                                                    Bldg Wood Column
                                                                
                                                                    Bldg Wood Columns
                                                                
                                                                    Bldg Wood Ext Wall
                                                                
                                                                    Bldg Wood Staircase,
                                                                
                                                                    Bldg-Mtd Lighting
                                                                
                                                                    Break Area
                                                                
                                                                    Break Area Base
                                                                
                                                                    Break Area Base Cabs w/Ctr
                                                                
                                                                    Break Area Base Cabs w/Ctr,
                                                                
                                                                    Break Area Dbl Sink
                                                                
                                                                    Break Area Dishwasher Electric,
                                                                
                                                                    Break Area Elec Water Heater
                                                                
                                                                    Break Area Equipment
                                                                
                                                                    Break Area Equipment Electrical
                                                                
                                                                    Break Area Gas Range/Oven
                                                                
                                                                    Break Area Sink
                                                                
                                                                    Break Area Wall
                                                                
                                                                    Break Area Wall Cabinets
                                                                
                                                                    Break Room Base Cabs w/Ctr
                                                                
                                                                    Break Room Equipment
                                                                
                                                                    Break Room Gas Range/Oven
                                                                
                                                                    Break Room Sink
                                                                
                                                                    Break Room Wall Cabinets
                                                                
                                                                    Breakfast Buffet Base
                                                                
                                                                    Breakfast Buffet Equipment
                                                                
                                                                    Building Foundation Wall
                                                                
                                                                    Building Spread Foundation
                                                                
                                                                    Building-Mounted
                                                                
                                                                    Building-Mounted Lighting
                                                                
                                                                    Built-In Desk,
                                                                
                                                                    Built-In Wardrobe,
                                                                
                                                                    CCTV Electrical,
                                                                
                                                                    Ceiling
                                                                
                                                                    Ceiling Fan Electrical,
                                                                
                                                                    Ceilings &amp; Partitions
                                                                
                                                                    Ceramic Tile
                                                                
                                                                    Chair Rail
                                                                
                                                                    Checkout Counter,
                                                                
                                                                    Chimney
                                                                
                                                                    Closet
                                                                
                                                                    Closet Door,
                                                                
                                                                    Cold Storage Ceiling Unit Cooler,
                                                                
                                                                    Cold Storage Condenser,
                                                                
                                                                    Cold Storage Drain Piping,
                                                                
                                                                    Cold Storage Electric,
                                                                
                                                                    Cold Storage Refrigeration Piping,
                                                                
                                                                    Cold Storage Remote Compressor,
                                                                
                                                                    Cold Storage,
                                                                
                                                                    Common
                                                                
                                                                    Common Area
                                                                
                                                                    Comms/Data Equipment
                                                                
                                                                    Comms/Data Equipment Electric
                                                                
                                                                    Compressed Air Equipment Electrical,
                                                                
                                                                    Compressed Air Equipment,
                                                                
                                                                    Compressed Air Piping,
                                                                
                                                                    Computer Equipment
                                                                
                                                                    Computer Equipment Electric
                                                                
                                                                    Computer Room HVAC
                                                                
                                                                    Concrete Topping Floor Finish
                                                                
                                                                    Condiments &amp; Beverage Counter,
                                                                
                                                                    Convenience Outlet,
                                                                
                                                                    Crane Equipment Electrical,
                                                                
                                                                    Crane Equipment,
                                                                
                                                                    Crane Rail Steel Support,
                                                                
                                                                    Crown Molding
                                                                
                                                                    Data Center Access Flooring, 
                                                                
                                                                    Data Center Electrical, 
                                                                
                                                                    Data Center Fire Suppression System, 
                                                                
                                                                    Data Center HVAC Electrical, 
                                                                
                                                                    Data Center HVAC, 
                                                                
                                                                    Data/Comms Equipment
                                                                
                                                                    Decorative
                                                                
                                                                    Decorative Pendant
                                                                
                                                                    Decorative Pendant Light
                                                                
                                                                    Decorative Pendant Lighting
                                                                
                                                                    Demountable Mezzanine
                                                                
                                                                    Demountable Partition, 
                                                                
                                                                    Demountable PTAC HVAC System Electrical, 
                                                                
                                                                    Demountable PTAC HVAC System, 
                                                                
                                                                    Detached Garage Concrete Slab
                                                                
                                                                    Detached Garage Ext Brick Veneer Wall
                                                                
                                                                    Detached Garage Ext Metal Siding Wall
                                                                
                                                                    Detached Garage Ext Stone Veneer Wall
                                                                
                                                                    Detached Garage Ext Vinyl Siding Wall
                                                                
                                                                    Detached Garage Ext Wall Lighting
                                                                
                                                                    Detached Garage Ext Wall Lighting Electrical
                                                                
                                                                    Detached Garage Ext Wood Siding Wall
                                                                
                                                                    Detached Garage Ext Wood Stud Fiber Cement Wall
                                                                
                                                                    Detached Garage Ext Wood Stud Stucco Wall
                                                                
                                                                    Detached Garage Exterior Doors
                                                                
                                                                    Detached Garage General Lighting
                                                                
                                                                    Detached Garage Roof Cover
                                                                
                                                                    Detached Garage Roof Drainage
                                                                
                                                                    Detached Garage Roof Structure
                                                                
                                                                    Detached Outdoor Deck,
                                                                
                                                                    Dishwasher Drain,
                                                                
                                                                    Dishwasher Electrical,
                                                                
                                                                    Dishwasher Hood Electrical,
                                                                
                                                                    Dishwasher Water Supply
                                                                
                                                                    Elec Water Cooler
                                                                
                                                                    Electric Water Cooler
                                                                
                                                                    Electric Water Cooler Electrical
                                                                
                                                                    Elev Wood Joist Floor,
                                                                
                                                                    Elevated Floors
                                                                
                                                                    Elevator
                                                                
                                                                    Elevator Electrical
                                                                
                                                                    Elevator Sump
                                                                
                                                                    Elevator Sump Pump
                                                                
                                                                    Elevator,
                                                                
                                                                    Emergency
                                                                
                                                                    Emergency Generator Equipment,
                                                                
                                                                    Emergency Lighting
                                                                
                                                                    Employee Break Area Base
                                                                
                                                                    Employee Break Area Wall
                                                                
                                                                    Employee Lounge
                                                                
                                                                    Equip Elec Inc SVC
                                                                
                                                                    Equipment Elec
                                                                
                                                                    Equipment Electric
                                                                
                                                                    Equipment Electric SVC
                                                                
                                                                    Equipment Protection Bollards,
                                                                
                                                                    Equipment Protection Guardrails,
                                                                
                                                                    Exam Room Base Cabs w/Ctr
                                                                
                                                                    Exam Room Equipment
                                                                
                                                                    Exam Room Equipment Wash Sink
                                                                
                                                                    Exam Room Plumbing
                                                                
                                                                    Exam Room Sink
                                                                
                                                                    Exam Room Wall Cabinets
                                                                
                                                                    Exit
                                                                
                                                                    Exit &amp; Emerg Lt Combo
                                                                
                                                                    Exit Lighting
                                                                
                                                                    Ext Garage
                                                                
                                                                    Ext Stairs
                                                                
                                                                    Exterior
                                                                
                                                                    Exterior Canopy Can Lighting
                                                                
                                                                    Exterior Concrete Stairs
                                                                
                                                                    Exterior Doors
                                                                
                                                                    Exterior Garage
                                                                
                                                                    Exterior Staircase
                                                                
                                                                    Exterior Storage Bldg Doors
                                                                
                                                                    Exterior Storage Bldg Interior Partitions
                                                                
                                                                    Exterior Storage Bldg Structure,
                                                                
                                                                    Exterior Wall EIFS Coating Finish,
                                                                
                                                                    Exterior Wall Paint Finish,
                                                                
                                                                    Exterior Wall Stucco Finish,
                                                                
                                                                    Exterior Window Treatment,
                                                                
                                                                    Exterior Wood Stairs
                                                                
                                                                    Eye Wash Stations
                                                                
                                                                    FF &amp; E,
                                                                
                                                                    Fire Alarm, 
                                                                
                                                                    Fire Sprinkler System
                                                                
                                                                    Fireplace
                                                                
                                                                    Floating Floor - Common Area
                                                                
                                                                    Food Prep Kitchen
                                                                
                                                                    Food Prep Kitchen Base
                                                                
                                                                    Food Prep Kitchen Equipment
                                                                
                                                                    Food Prep Kitchen Sink
                                                                
                                                                    Food Prep Kitchen Wall
                                                                
                                                                    Front Desk w/Low Wall,
                                                                
                                                                    Front Entry
                                                                
                                                                    FRP Wall Covering,
                                                                
                                                                    Fryer Electrical,
                                                                
                                                                    Fume Hoods
                                                                
                                                                    Gas Dryer Electrical,
                                                                
                                                                    Gas Fryer
                                                                
                                                                    Gas Griddle
                                                                
                                                                    Gas Griddle &amp; Oven Combo
                                                                
                                                                    Gas Range
                                                                
                                                                    Gas Range &amp; Oven Combo
                                                                
                                                                    Gas Range, Griddle, &amp; Oven Combo
                                                                
                                                                    Gen Elec Inc SVC
                                                                
                                                                    Gen Gas WH
                                                                
                                                                    General
                                                                
                                                                    General Bathtub
                                                                
                                                                    General Domestic Water
                                                                
                                                                    General Electric
                                                                
                                                                    General Electric SVC
                                                                
                                                                    General Electric Water Heater
                                                                
                                                                    General Electric Water Htr
                                                                
                                                                    General Equipment Piping
                                                                
                                                                    General Gas
                                                                
                                                                    General Gas Pipe,
                                                                
                                                                    General Gas Water Heater
                                                                
                                                                    General Lavatory
                                                                
                                                                    General Lighting
                                                                
                                                                    General Lighting Electrical,
                                                                
                                                                    General San Sewer
                                                                
                                                                    General Sanitary
                                                                
                                                                    General Sanitary Sewer Pipe
                                                                
                                                                    General Sanitary Sewer Vent Pipe,
                                                                
                                                                    General Sanitary Sewer,
                                                                
                                                                    General Sanitary Vent,
                                                                
                                                                    General Shower
                                                                
                                                                    General SVC Sink
                                                                
                                                                    General Urinal
                                                                
                                                                    General Water
                                                                
                                                                    General Water Closet
                                                                
                                                                    General Water Heater
                                                                
                                                                    General Water Heater Gas Pipe,
                                                                
                                                                    General Water Heater Water Pipe,
                                                                
                                                                    General Water Pipe,
                                                                
                                                                    General WC
                                                                
                                                                    Griddle &amp; Oven Combo Electrical,
                                                                
                                                                    Griddle Electrical,
                                                                
                                                                    Guest Room Coffee Maker Electric
                                                                
                                                                    Guest Room Electrical
                                                                
                                                                    Guest Room Furniture,
                                                                
                                                                    Guest Room Hair Dryer Electric
                                                                
                                                                    Guest Room Microwave Electric
                                                                
                                                                    Guest Room Microwave,
                                                                
                                                                    Guest Room Mini Fridge Electric
                                                                
                                                                    Guest Room Mini Fridge,
                                                                
                                                                    Guest Room Refrigerator,
                                                                
                                                                    Guest Room Television,
                                                                
                                                                    Handicap Bathroom Accessories,
                                                                
                                                                    Handicap Lavatory,
                                                                
                                                                    Handicap Shower,
                                                                
                                                                    Handsink Drain
                                                                
                                                                    Handsink Water Supply
                                                                
                                                                    Hostess Station Counter,
                                                                
                                                                    Hot Tub Electrical
                                                                
                                                                    HVAC
                                                                
                                                                    HVAC AHU
                                                                
                                                                    HVAC Drain Piping,
                                                                
                                                                    HVAC Electric
                                                                
                                                                    HVAC Electric Unit Heater
                                                                
                                                                    HVAC Electrical
                                                                
                                                                    HVAC Gas Furnace
                                                                
                                                                    HVAC Gas Piping
                                                                
                                                                    HVAC Gas Piping,
                                                                
                                                                    HVAC RTU
                                                                
                                                                    HVAC Split System
                                                                
                                                                    HVAC Split System Elec Heater
                                                                
                                                                    HVAC Split System Rmt Condenser
                                                                
                                                                    HVAC SS
                                                                
                                                                    HVAC SS AHU
                                                                
                                                                    HVAC SS Rmt Cond
                                                                
                                                                    Hydraulic Equipment
                                                                
                                                                    Ice Cream Machine Electrical,
                                                                
                                                                    Inc Gen Elec SVC
                                                                
                                                                    Indoor
                                                                
                                                                    Indoor Coiling Door
                                                                
                                                                    Indoor Drinking Fountains,
                                                                
                                                                    Indoor Fireplace,
                                                                
                                                                    Indoor Hot Tub Electric Water Heater,
                                                                
                                                                    Indoor Hot Tub Equipment Electrical,
                                                                
                                                                    Indoor Hot Tub Gas Piping,
                                                                
                                                                    Indoor Hot Tub Gas Water Heater,
                                                                
                                                                    Indoor Hot Tub Water Piping,
                                                                
                                                                    Indoor Jacuzzi
                                                                
                                                                    Indoor Jacuzzi Equipment Electrical,
                                                                
                                                                    Indoor Jacuzzi Gas Piping,
                                                                
                                                                    Indoor Jacuzzi Heater,
                                                                
                                                                    Indoor Jacuzzi Piping,
                                                                
                                                                    Indoor Jacuzzi Water Supply Piping,
                                                                
                                                                    Indoor Jacuzzi,
                                                                
                                                                    Indoor Plant Wall Water
                                                                
                                                                    Indoor Pool Deck
                                                                
                                                                    Indoor Pool Dehumdifier,
                                                                
                                                                    Indoor Pool Dehumidifer,
                                                                
                                                                    Indoor Pool Equipment
                                                                
                                                                    Indoor Pool Equipment Electrical,
                                                                
                                                                    Indoor Pool Equipment Gas Piping,
                                                                
                                                                    Indoor Pool Exhaust
                                                                
                                                                    Indoor Pool Piping,
                                                                
                                                                    Indoor Pool Water Supply Piping,
                                                                
                                                                    Indoor Sauna Equipment
                                                                
                                                                    Indoor Sauna Equipment Electrical
                                                                
                                                                    Indoor Saunas
                                                                
                                                                    Indoor Steam Room Door,
                                                                
                                                                    Indoor Steam Room Drain Piping,
                                                                
                                                                    Indoor Steam Room Equipment
                                                                
                                                                    Indoor Steam Room Equipment Electrical
                                                                
                                                                    Indoor Steam Room Gas Piping,
                                                                
                                                                    Indoor Steam Room Piping
                                                                
                                                                    Indoor Steam Room Water Supply Piping,
                                                                
                                                                    Indoor Steam Rooms
                                                                
                                                                    Indoor Vertical Lift Door
                                                                
                                                                    Indoor Vertical Lift OHead
                                                                
                                                                    Int Inc Equip Elec SVC
                                                                
                                                                    Int Inc Gen Elec SVC
                                                                
                                                                    Int of Ext Wall Drywall Sheathing
                                                                
                                                                    Int of Exterior Wall
                                                                
                                                                    Int Sec Coiling Door,
                                                                
                                                                    Interior Closet
                                                                
                                                                    Interior Door
                                                                
                                                                    Interior Door,
                                                                
                                                                    Interior Doors
                                                                
                                                                    Interior Driveway
                                                                
                                                                    Interior Partition
                                                                
                                                                    Interior Partition,
                                                                
                                                                    Interior Partitions
                                                                
                                                                    Interior Perimeter Wall,
                                                                
                                                                    Interior Rollup Door,
                                                                
                                                                    Interior Staircase
                                                                
                                                                    Interior Storage Doors
                                                                
                                                                    Interior Storage Partitions
                                                                
                                                                    Interior Window Treatment,
                                                                
                                                                    Kitchen
                                                                
                                                                    Kitchen 3-Comp Sink
                                                                
                                                                    Kitchen Base Cabs
                                                                
                                                                    Kitchen Base Cabs w/Ctr
                                                                
                                                                    Kitchen Base Cabs,
                                                                
                                                                    Kitchen Built-In
                                                                
                                                                    Kitchen Counter
                                                                
                                                                    Kitchen Counter,
                                                                
                                                                    Kitchen Dishwasher Electric,
                                                                
                                                                    Kitchen Elec Range/Oven
                                                                
                                                                    Kitchen Equip Electrical
                                                                
                                                                    Kitchen Equipment
                                                                
                                                                    Kitchen Equipment Electric
                                                                
                                                                    Kitchen Equipment Electrical
                                                                
                                                                    Kitchen Exhaust
                                                                
                                                                    Kitchen Gas Piping,
                                                                
                                                                    Kitchen Gas Range
                                                                
                                                                    Kitchen Gas Range/Oven
                                                                
                                                                    Kitchen Gas Range/Oven,
                                                                
                                                                    Kitchen Main Gas
                                                                
                                                                    Kitchen Oven
                                                                
                                                                    Kitchen Range Electrical
                                                                
                                                                    Kitchen Range Electrical,
                                                                
                                                                    Kitchen Sink
                                                                
                                                                    Kitchen Sink,
                                                                
                                                                    Kitchen Wall
                                                                
                                                                    Kitchen Wall Cabs,
                                                                
                                                                    Lab Sink
                                                                
                                                                    Laboratory Base Cabs/w Ctrs,
                                                                
                                                                    Laboratory Electric
                                                                
                                                                    Laboratory Wall
                                                                
                                                                    Laminate
                                                                
                                                                    Laminate Flooring,
                                                                
                                                                    Landscape
                                                                
                                                                    Laundry
                                                                
                                                                    Laundry Base
                                                                
                                                                    Laundry Drain Piping,
                                                                
                                                                    Laundry Dryer,
                                                                
                                                                    Laundry Elec Dryer
                                                                
                                                                    Laundry Electric Dryer
                                                                
                                                                    Laundry Electric Water Heater
                                                                
                                                                    Laundry Equipment
                                                                
                                                                    Laundry Equipment Electrical
                                                                
                                                                    Laundry Gas Dryer
                                                                
                                                                    Laundry Gas Piping,
                                                                
                                                                    Laundry Piping
                                                                
                                                                    Laundry Sink
                                                                
                                                                    Laundry Wall
                                                                
                                                                    Laundry Washer
                                                                
                                                                    Laundry Washer,
                                                                
                                                                    Laundry Waste Pipe
                                                                
                                                                    Laundry Water Pipe
                                                                
                                                                    Laundry Water Piping,
                                                                
                                                                    LVT
                                                                
                                                                    Make-up Air Electrical,
                                                                
                                                                    Manufacturing Equipment Electrical,
                                                                
                                                                    Manufacturing Equipment Protection Bollards,
                                                                
                                                                    Manufacturing Equipment Protection Guardrails,
                                                                
                                                                    Manufacturing Equipment Water Piping,
                                                                
                                                                    Medical Equipment Electric
                                                                
                                                                    Medical Office Electric Water Heater
                                                                
                                                                    Medical Office Fixture Waste
                                                                
                                                                    Medical Office Fixture Water
                                                                
                                                                    Medical Office Gas Water Heater
                                                                
                                                                    Natural
                                                                
                                                                    Nurse's Station w/Low Wall Ctr
                                                                
                                                                    Nurses' Station Back Base Cabs w/Ctr
                                                                
                                                                    Nurses' Station Desk w/Low Wall Ctr
                                                                
                                                                    Nurses' Station Low Wall w.Ctr
                                                                
                                                                    Nurses' Station Sink
                                                                
                                                                    Nurses' Station Wall
                                                                
                                                                    Nurses' Station Wall Cabinet
                                                                
                                                                    Nurses' Station Wall Cabinets
                                                                
                                                                    Office Base Cabinetry,
                                                                
                                                                    Office Base Cabs w/Ctr
                                                                
                                                                    Office Furniture Electric
                                                                
                                                                    Office Wall
                                                                
                                                                    Office Wall Cabinetry,
                                                                
                                                                    Outdoor Deck,
                                                                
                                                                    Parking Garage
                                                                
                                                                    Parking Garage Bollards
                                                                
                                                                    Parking Garage CC Column
                                                                
                                                                    Parking Garage CC Footings
                                                                
                                                                    Parking Garage CC Slab
                                                                
                                                                    Parking Garage Drainage
                                                                
                                                                    Parking Garage Elev Concrete Floor
                                                                
                                                                    Parking Garage Elevator Electrical,
                                                                
                                                                    Parking Garage Elevator,
                                                                
                                                                    Parking Garage Ext Brick Wall
                                                                
                                                                    Parking Garage Ext CMU Stucco Wall
                                                                
                                                                    Parking Garage Ext CMU Wall
                                                                
                                                                    Parking Garage Ext Glass Curtain Wall
                                                                
                                                                    Parking Garage Ext Metal Panel Curtain Wall
                                                                
                                                                    Parking Garage Ext Stone Veneer Wall
                                                                
                                                                    Parking Garage Exterior Doors
                                                                
                                                                    Parking Garage Lighting
                                                                
                                                                    Parking Garage Sprinkler System
                                                                
                                                                    Parking Garage Steel Railing
                                                                
                                                                    Parking Garage Steel Railing,
                                                                
                                                                    Pier and Beam Foundation CC Footing,
                                                                
                                                                    Pier and Beam Foundation Excavation,
                                                                
                                                                    Pier and Beam Foundation Wood Columns,
                                                                
                                                                    Pier and Beam Wood Floor Construction,
                                                                
                                                                    PoS Equipment
                                                                
                                                                    Pot Sink Drain
                                                                
                                                                    Pot Sink Water Supply
                                                                
                                                                    Prescription Drug Refrigeration Equip
                                                                
                                                                    Process Area Electric Water Heater,
                                                                
                                                                    Process Area Gas Water Heater,
                                                                
                                                                    Process Area Roof Exhaust Electrical,
                                                                
                                                                    Process Area Roof Exhaust,
                                                                
                                                                    Process Area Trench Drains,
                                                                
                                                                    Process Area Wall Exhaust Electrical,
                                                                
                                                                    Process Area Wall Exhaust,
                                                                
                                                                    Process Area Water Heater Electrical,
                                                                
                                                                    Process Area Water Heater Piping,
                                                                
                                                                    PTAC HVAC Drain Piping,
                                                                
                                                                    PVC Wall Covering,
                                                                
                                                                    Range &amp; Oven Combo Electrical,
                                                                
                                                                    Range Electrical,
                                                                
                                                                    Range, Griddle, &amp; Oven Combo Electrical,
                                                                
                                                                    Reach-in Refrigerator Electrical,
                                                                
                                                                    Reception Back Base Cabinetry
                                                                
                                                                    Reception Back Wall Cabinetry
                                                                
                                                                    Reception Built-in Desk
                                                                
                                                                    Reception Desk Glass Wall,
                                                                
                                                                    Reception Desk w/Low Wall, 
                                                                
                                                                    Refrigeration Equipment
                                                                
                                                                    Refrigeration Room Electric,
                                                                
                                                                    Removable Mezzanine
                                                                
                                                                    Residential Unit
                                                                
                                                                    Restroom
                                                                
                                                                    Restroom Counter
                                                                
                                                                    Restroom Countertop,
                                                                
                                                                    Restroom Electric,
                                                                
                                                                    Restroom Exhaust
                                                                
                                                                    Restroom Hand Dryer
                                                                
                                                                    Restroom Lavatory,
                                                                
                                                                    Restroom Mirror
                                                                
                                                                    Restroom Urinal,
                                                                
                                                                    Restroom Vanity
                                                                
                                                                    Restroom Vanity Base Cabinets,
                                                                
                                                                    Restroom Vanity Base Cabs w/Ctr
                                                                
                                                                    Restroom Vanity Countertop,
                                                                
                                                                    Restroom Water Closet,
                                                                
                                                                    Roof Drain
                                                                
                                                                    Roof-Mtd Photovoltaic System
                                                                
                                                                    Sanitary
                                                                
                                                                    Security
                                                                
                                                                    Server Room HVAC
                                                                
                                                                    Sheet Vinyl,
                                                                
                                                                    Shower Enclosures,
                                                                
                                                                    Site
                                                                
                                                                    Site Asphalt
                                                                
                                                                    Site Asphalt Berm
                                                                
                                                                    Site Asphalt Driveway
                                                                
                                                                    Site Asphalt Paving
                                                                
                                                                    Site Barbecue Grille
                                                                
                                                                    Site Basketball
                                                                
                                                                    Site Bollards,
                                                                
                                                                    Site Brick Pavers
                                                                
                                                                    Site Carports,
                                                                
                                                                    Site CC Curb &amp; Gutter
                                                                
                                                                    Site CC Driveway
                                                                
                                                                    Site CC Patio
                                                                
                                                                    Site CC Sidewalk
                                                                
                                                                    Site CC Slab on Grade
                                                                
                                                                    Site CC Truck Ramp
                                                                
                                                                    Site Chain Link
                                                                
                                                                    Site Concrete Curb
                                                                
                                                                    Site Concrete Curb &amp; Gutter
                                                                
                                                                    Site Concrete Patio
                                                                
                                                                    Site Concrete Paving
                                                                
                                                                    Site Concrete Sidewalk
                                                                
                                                                    Site Concrete Walkways
                                                                
                                                                    Site Decorative Pole Lighting
                                                                
                                                                    Site Driveway Pavers
                                                                
                                                                    Site Dumpster Enclosure
                                                                
                                                                    Site Equipment Protection Bollards,
                                                                
                                                                    Site Fencing
                                                                
                                                                    Site Fire Hydrant
                                                                
                                                                    Site Flagpoles,
                                                                
                                                                    Site Gravel Driveway
                                                                
                                                                    Site Gravel Walkways
                                                                
                                                                    Site Guardrail,
                                                                
                                                                    Site Handrails,
                                                                
                                                                    Site Hot Tub
                                                                
                                                                    Site Hot Tub Equipment
                                                                
                                                                    Site Hot Tub Equipment Electrical
                                                                
                                                                    Site Hot Tub Equipment Piping
                                                                
                                                                    Site Hot Tub Pad
                                                                
                                                                    Site Incoming Electric SVC
                                                                
                                                                    Site Incoming Fire Protection SVC
                                                                
                                                                    Site Incoming N Gas SVC
                                                                
                                                                    Site Incoming San Sewer SVC
                                                                
                                                                    Site Incoming Water SVC
                                                                
                                                                    Site Irrigation
                                                                
                                                                    Site Jacuzzi
                                                                
                                                                    Site Jacuzzi Heater
                                                                
                                                                    Site Jacuzzi,
                                                                
                                                                    Site Landscape
                                                                
                                                                    Site Landscaping
                                                                
                                                                    Site Parking Curb,
                                                                
                                                                    Site Parking Curbs,
                                                                
                                                                    Site Parking Signs,
                                                                
                                                                    Site Patio Fencing,
                                                                
                                                                    Site Patio Pavers
                                                                
                                                                    Site Planter,
                                                                
                                                                    Site Pole Lighting
                                                                
                                                                    Site Porte Cochere Brick Veneer Finish
                                                                
                                                                    Site Porte Cochere Column Covering
                                                                
                                                                    Site Porte Cochere Concrete Footing
                                                                
                                                                    Site Porte Cochere EIFS Finish
                                                                
                                                                    Site Porte Cochere Lighting
                                                                
                                                                    Site Porte Cochere Roof Cover
                                                                
                                                                    Site Porte Cochere Roof Drainage
                                                                
                                                                    Site Porte Cochere Roof Structure
                                                                
                                                                    Site Porte Cochere Steel Beams
                                                                
                                                                    Site Porte Cochere Steel Columns
                                                                
                                                                    Site Porte Cochere Stone Veneer Finish
                                                                
                                                                    Site Porte Cochere Stucco Finish
                                                                
                                                                    Site Porte Cochere Wood Beams
                                                                
                                                                    Site Porte Cochere Wood Columns
                                                                
                                                                    Site Privacy Wall
                                                                
                                                                    Site Reinf CC Paving
                                                                
                                                                    Site Retaining Wall
                                                                
                                                                    Site Slate Pavers
                                                                
                                                                    Site Storm Drainage
                                                                
                                                                    Site Storm Water Detention Area
                                                                
                                                                    Site Swimming Pool
                                                                
                                                                    Site Swimming Pool Deck
                                                                
                                                                    Site Swimming Pool Enclosure
                                                                
                                                                    Site Swimming Pool Equipment
                                                                
                                                                    Site Swimming Pool Equipment Electrical
                                                                
                                                                    Site Swimming Pool Equipment Piping
                                                                
                                                                    Site Walkway Pavers
                                                                
                                                                    Site Water Line
                                                                
                                                                    Soiled Utility Sink
                                                                
                                                                    Sprinkler System
                                                                
                                                                    Stone Tile Finishing,
                                                                
                                                                    Storage Base Cabs w/Ctrs,
                                                                
                                                                    Storage Wall
                                                                
                                                                    Storefront Doors,
                                                                
                                                                    Storefront Windows,
                                                                
                                                                    Tall Storage Cabinets
                                                                
                                                                    Tenant Kitchen Spaces
                                                                
                                                                    Tenant Kitchen Spaces Gas WH
                                                                
                                                                    Tenant Space Finishes
                                                                
                                                                    Tenant Spaces Food Prep 3-Comp Sink
                                                                
                                                                    Terrace-Mounted Light Bollard
                                                                
                                                                    Vanity Base Cabinet,
                                                                
                                                                    Vanity Cabinet Countertop,
                                                                
                                                                    Vanity Countertop,
                                                                
                                                                    Vanity Sink,
                                                                
                                                                    VCT
                                                                
                                                                    VCT,
                                                                
                                                                    Vent Kit Electrical,
                                                                
                                                                    Vinyl Sheet Flooring,
                                                                
                                                                    Vinyl Wall Covering,
                                                                
                                                                    Walk-in Cooler Drain,
                                                                
                                                                    Walk-in Cooler Electric,
                                                                
                                                                    Walk-in Cooler,
                                                                
                                                                    Walk-in Freezer Drain,
                                                                
                                                                    Walk-in Freezer Electric,
                                                                
                                                                    Walk-in Freezer,
                                                                
                                                                    Walk-in Refrigerator Drain,
                                                                
                                                                    Walk-in Refrigerator Electrical,
                                                                
                                                                    Walk-in Refrigerator,
                                                                
                                                                    Wall
                                                                
                                                                    Wall Paneling,
                                                                
                                                                    Wall Tiles,
                                                                
                                                                    Wallpaper Wall Covering,
                                                                
                                                                    Wallpaper,
                                                                
                                                                    Water Distribution Piping,
                                                                
                                                                    Water Heater Electrical
                                                                
                                                                    Water Heater Gas Piping,
                                                                
                                                                    Water Heater Piping
                                                                
                                                                    Water Heater Piping,
                                                                
                                                                    Water Heater Water Piping,
                                                                
                                                                    Wet Bar
                                                                
                                                                    Wet Bar Sink
                                                                
                                                                    Window Shades
                                                                
                                                                    Window Treatment
                                                                
                                                                    Windows
                                                                
                                                                    Wood Flooring
                                                                
                                                                    Wood Flooring,
                                                                
                                                                    Wood Wall Paneling,
                                                                
                                                                    Work Room Base Cabs w/Ctr
                                                                
                                                                    Work Room Wall Cabinets
                                                                
                                                                    X-Ray Equipment
                                                                
                                                                    X-Ray Equipment Electrical
                                                                
                                                                    X-Ray Shielded Doors
                                                                
                                                                    X-Ray Shielded Partition
                                                                
                                                                    X-Ray Shielded Window
                                                                
                                                                    Add Additional Purpose Description
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Purpose Description
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            Location
                                                        
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                                    Building
                                                                
                                                                    Building Interior
                                                                
                                                                    Site Exterior
                                                                
                                                                    Add Additional Location
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Building
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Location
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            Asset Class
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                                    1 - GENERAL REQUIREMENTS
                                                                
                                                                    2 - SITEWORK
                                                                
                                                                    3 - CONCRETE
                                                                
                                                                    4 - MASONRY
                                                                
                                                                    5 - METALS
                                                                
                                                                    6 - WOOD &amp; PLASTICS
                                                                
                                                                    7 - THERMAL &amp; MOISTURE PROTECTION
                                                                
                                                                    8 - DOORS &amp; WINDOWS
                                                                
                                                                    9 - FINISHES
                                                                
                                                                    10 - SPECIALTIES
                                                                
                                                                    11 - EQUIPMENT
                                                                
                                                                    12 - FURNISHINGS
                                                                
                                                                    13 - SPECIAL CONSTRUCTION
                                                                
                                                                    14 - CONVEYING SYSTEMS
                                                                
                                                                    15 - PLUMBING
                                                                
                                                                    16 - MECHANICAL
                                                                
                                                                    17 - HVAC
                                                                
                                                                    18 - ELECTRICAL
                                                                
                                                                    19 - BUILDING GAS SYSTEM
                                                                
                                                                    20 - BUILDING FIRE PROTECTION
                                                                
                                                                    21 - SITE FIRE PROTECTION
                                                                
                                                                    30 - ATM ELECTRICAL
                                                                
                                                                    32 - AUDIO/VISUAL SYSTEM
                                                                
                                                                    34 - AUTOMATIC DOOR OPERATOR
                                                                
                                                                    36 - AWNINGS &amp; CANOPIES
                                                                
                                                                    38 - BEVERAGE DISPENSER ELECTRICAL
                                                                
                                                                    40 - BEVERAGE DISPENSER PLUMBING
                                                                
                                                                    42 - BREAK ROOM ELECTRICAL
                                                                
                                                                    44 - BREAK ROOM PLUMBING
                                                                
                                                                    46 - BUILDING SIGNAGE
                                                                
                                                                    48 - BUILDING SIGNAGE ELECTRICAL
                                                                
                                                                    50 - COFFEE BAR ELECTRICAL
                                                                
                                                                    52 - COFFEE BAR PLUMBING
                                                                
                                                                    54 - COMPRESSED AIR SYSTEM ELECTRICAL
                                                                
                                                                    56 - COMPUTER CONNECTIONS
                                                                
                                                                    58 - COPIER ELECTRICAL
                                                                
                                                                    60 - CURBING &amp; BUMPERS, SITE
                                                                
                                                                    61 - DEMOUNTABLE HVAC SYSTEMS
                                                                
                                                                    61.5 - DEMOUNTABLE MEZZANINE
                                                                
                                                                    62 - DOCK EQUIPMENT
                                                                
                                                                    64 - DOOR, INTERIOR SPECIALTY
                                                                
                                                                    66 - ELECTRICAL, EQUIPMENT
                                                                
                                                                    67 - EMERGENCY GENERATOR SYSTEM
                                                                
                                                                    68 - EMERGENCY EYE WASH
                                                                
                                                                    70 - FENCING, INTERIOR
                                                                
                                                                    72 - FENCING, SITE
                                                                
                                                                    74 - FIRE EXTINGUISHERS
                                                                
                                                                    76 - FITNESS EQUIPMENT ELECTRICAL
                                                                
                                                                    78 - FLAGPOLES, SITE
                                                                
                                                                    80 - FLOORING, REMOVABLE
                                                                
                                                                    82 - FURNISHINGS, SITE
                                                                
                                                                    84 - FURNITURE, FIXTURES, &amp; EQUIPMENT
                                                                
                                                                    86 - GARAGE DOOR OPERATOR
                                                                
                                                                    88 - GATE OPERATOR
                                                                
                                                                    90 - GLUED-ON FINISHES
                                                                
                                                                    91 - INDOOR SWIMMING POOL EQUIPMENT
                                                                
                                                                    92 - KITCHEN EQUIPMENT
                                                                
                                                                    94 - KITCHEN ELECTRICAL
                                                                
                                                                    96 - KITCHEN PLUMBING
                                                                
                                                                    98 - LANDSCAPING &amp; IRRIGATION
                                                                
                                                                    100 - LAUNDRY APPLIANCES
                                                                
                                                                    102 - LAUNDRY ELECTRICAL
                                                                
                                                                    104 - LAUNDRY PLUMBING
                                                                
                                                                    106 - LIGHTING, DECORATIVE INTERIOR
                                                                
                                                                    108 - LIGHTING, DECORATIVE SITE
                                                                
                                                                    110 - LIGHTING, EXIT &amp; EMERGENCY
                                                                
                                                                    112 - LIGHTING, EXTERIOR DECORATIVE
                                                                
                                                                    114 - LIGHTING, PARKING LOT
                                                                
                                                                    116 - LOADING DOCK, SITE
                                                                
                                                                    118 - LOCKERS
                                                                
                                                                    120 - MAILBOXES
                                                                
                                                                    122 - MILLWORK
                                                                
                                                                    123 - MOVABLE PARTITIONS
                                                                
                                                                    124 - OIL SYSTEM PLUMBING
                                                                
                                                                    125 - PARKING TICKETING EQUIPMENT
                                                                
                                                                    126 - PARTITIONS, SPECIALTY
                                                                
                                                                    128 - PARTS DEPARTMENT ELECTRICAL
                                                                
                                                                    130 - PAVING, WALKS &amp; ROADS
                                                                
                                                                    132 - PHOTOVOLTAIC SYSTEM
                                                                
                                                                    134 - PIPE BOLLARDS, INTERIOR
                                                                
                                                                    136 - PIPE BOLLARDS, SITE
                                                                
                                                                    138 - PLUMBING, EQUIPMENT
                                                                
                                                                    140 - REFRIGERATION ELECTRIC
                                                                
                                                                    142 - REFRIGERATION EQUIPMENT
                                                                
                                                                    145 - SECURITY SYSTEMS
                                                                
                                                                    146 - SERVER ROOM HVAC
                                                                
                                                                    148 - SHOP CRANE
                                                                
                                                                    150 - SHOP ELECTRICAL
                                                                
                                                                    152 - SHOP EQUIPMENT STRUCTURE
                                                                
                                                                    154 - SHOP EXHAUST SYSTEM
                                                                
                                                                    156 - SHOP PLUMBING
                                                                
                                                                    158 - SIGNAGE, SITE
                                                                
                                                                    159 - SITE ATHLETICS COURTS
                                                                
                                                                    160 - SITE FURNISHING GAS CONNECTIONS
                                                                
                                                                    161 - SITE PORTE COCHERE
                                                                
                                                                    162 - SITE PRIVACY WALLS
                                                                
                                                                    163 - SITE RETAINING WALLS
                                                                
                                                                    164 - SITE SIGNAGE ELECTRICAL
                                                                
                                                                    166 - SITE SWIMMING POOLS
                                                                
                                                                    166.5 - SPECIALTY CONVEYING SYSTEMS
                                                                
                                                                    167 - SPECIALTY FIRE SUPPRESSION
                                                                
                                                                    168 - STORM DRAIN SYSTEM
                                                                
                                                                    170 - SURVEILLANCE SYSTEM
                                                                
                                                                    172 - SURVEILLANCE SYSTEM CONNECTIONS
                                                                
                                                                    174 - TELEPHONE SYSTEM
                                                                
                                                                    176 - TELEPHONE SYSTEM CONNECTIONS
                                                                
                                                                    178 - TELEVISION / CABLE CONNECTIONS
                                                                
                                                                    180 - TRASH ENCLOSURE
                                                                
                                                                    182 - VENDING EQUIPMENT ELECTRICAL
                                                                
                                                                    184 - WALL PROTECTION / CORNER GUARDS
                                                                
                                                                    186 - WASH BAY ELECTRICAL
                                                                
                                                                    188 - WASH BAY PLUMBING
                                                                
                                                                    190 - WINDOW COVERINGS
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    
                                                        
                                                            
                                                                Takeoff Cost
                                                            
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    

                                                    
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #1
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #2
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #3
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #4
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #5
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            
                                                                Current Deprecation Method
                                                            
                                                        
                                                        
                                                            
                                                              
                                                              
                                                                None - Replacement Cost New
                                                              
                                                            
                                                            
                                                              
                                                              
                                                               Choose from below
                                                              
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            
                                                                
                                                                    Condition of Property
                                                                
                                                            
                                                            
                                                              
                                                                  Excellent - 90%
                                                                  Good - 75%
                                                                  Fair - 50%
                                                                  Bad - 25%
                                                                  Replacement Ready - 10%
                                                                  Add Custom Condition
                                                              
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    Custom Condition of Property
                                                                
                                                            
                                                            
                                                                
                                                            
                                                             . 
                                                            
                                                                
                                                            
                                                            
                                                                
                                                                 % 
                                                            
                                                            
                                                        
                                                        
                                                            
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                            
                                            
                                                
                                                
                                                
                                                    Save and add another Item/Assembly
                                                    Save and Add Same Item/Assembly Again
                                                    Save
                                                    Close
                                                    
                                                
                                            
                                        
                                        
                                        
                                    
                                
                            
                        
                        
                            
                            
                                
                                    
                                        
                                        
                                            
                                                
                                                    
                                                        *Description
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        *Unit Cost
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Takeoff Cost
                                                        
                                                            
                                                            

                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Quantity
                                                        
                                                            
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Total Cost
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        Cost Source
                                                        
                                                            
                                                                Select
                                                                
                                                                    Contractor Cost
                                                                
                                                                    Client Cost
                                                                
                                                                    RS Means
                                                                
                                                                    Site Contractor Cost
                                                                
                                                                    Site Indirect
                                                                
                                                                    Client Cost - Expenses
                                                                
                                                                Add Additional Cost Source
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Cost Source
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Cost Unit
                                                        
                                                            
                                                                Select
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    SFF
                                                                
                                                                Add Additional Cost Unit
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Cost Unit
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Location
                                                        
                                                            
                                                                Select
                                                                
                                                                    Building
                                                                
                                                                    Building Interior
                                                                
                                                                    Site Exterior
                                                                
                                                                    Add Additional location
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        Building
                                                        
                                                            Select
                                                            
                                                        
                                                        
                                                    
                                                    
                                                        
                                                            Extra Location
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        Recovery Period
                                                        
                                                            
                                                                
                                                                    5 Years
                                                                
                                                                    15 Years
                                                                
                                                                    39 Years
                                                                
                                                                    Indirect Costs
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Asset Class
                                                        
                                                            
                                                                Select
                                                                
                                                                    1 - GENERAL REQUIREMENTS
                                                                
                                                                    2 - SITEWORK
                                                                
                                                                    3 - CONCRETE
                                                                
                                                                    4 - MASONRY
                                                                
                                                                    5 - METALS
                                                                
                                                                    6 - WOOD &amp; PLASTICS
                                                                
                                                                    7 - THERMAL &amp; MOISTURE PROTECTION
                                                                
                                                                    8 - DOORS &amp; WINDOWS
                                                                
                                                                    9 - FINISHES
                                                                
                                                                    10 - SPECIALTIES
                                                                
                                                                    11 - EQUIPMENT
                                                                
                                                                    12 - FURNISHINGS
                                                                
                                                                    13 - SPECIAL CONSTRUCTION
                                                                
                                                                    14 - CONVEYING SYSTEMS
                                                                
                                                                    15 - PLUMBING
                                                                
                                                                    16 - MECHANICAL
                                                                
                                                                    17 - HVAC
                                                                
                                                                    18 - ELECTRICAL
                                                                
                                                                    19 - BUILDING GAS SYSTEM
                                                                
                                                                    20 - BUILDING FIRE PROTECTION
                                                                
                                                                    21 - SITE FIRE PROTECTION
                                                                
                                                                    30 - ATM ELECTRICAL
                                                                
                                                                    32 - AUDIO/VISUAL SYSTEM
                                                                
                                                                    34 - AUTOMATIC DOOR OPERATOR
                                                                
                                                                    36 - AWNINGS &amp; CANOPIES
                                                                
                                                                    38 - BEVERAGE DISPENSER ELECTRICAL
                                                                
                                                                    40 - BEVERAGE DISPENSER PLUMBING
                                                                
                                                                    42 - BREAK ROOM ELECTRICAL
                                                                
                                                                    44 - BREAK ROOM PLUMBING
                                                                
                                                                    46 - BUILDING SIGNAGE
                                                                
                                                                    48 - BUILDING SIGNAGE ELECTRICAL
                                                                
                                                                    50 - COFFEE BAR ELECTRICAL
                                                                
                                                                    52 - COFFEE BAR PLUMBING
                                                                
                                                                    54 - COMPRESSED AIR SYSTEM ELECTRICAL
                                                                
                                                                    56 - COMPUTER CONNECTIONS
                                                                
                                                                    58 - COPIER ELECTRICAL
                                                                
                                                                    60 - CURBING &amp; BUMPERS, SITE
                                                                
                                                                    61 - DEMOUNTABLE HVAC SYSTEMS
                                                                
                                                                    61.5 - DEMOUNTABLE MEZZANINE
                                                                
                                                                    62 - DOCK EQUIPMENT
                                                                
                                                                    64 - DOOR, INTERIOR SPECIALTY
                                                                
                                                                    66 - ELECTRICAL, EQUIPMENT
                                                                
                                                                    67 - EMERGENCY GENERATOR SYSTEM
                                                                
                                                                    68 - EMERGENCY EYE WASH
                                                                
                                                                    70 - FENCING, INTERIOR
                                                                
                                                                    72 - FENCING, SITE
                                                                
                                                                    74 - FIRE EXTINGUISHERS
                                                                
                                                                    76 - FITNESS EQUIPMENT ELECTRICAL
                                                                
                                                                    78 - FLAGPOLES, SITE
                                                                
                                                                    80 - FLOORING, REMOVABLE
                                                                
                                                                    82 - FURNISHINGS, SITE
                                                                
                                                                    84 - FURNITURE, FIXTURES, &amp; EQUIPMENT
                                                                
                                                                    86 - GARAGE DOOR OPERATOR
                                                                
                                                                    88 - GATE OPERATOR
                                                                
                                                                    90 - GLUED-ON FINISHES
                                                                
                                                                    91 - INDOOR SWIMMING POOL EQUIPMENT
                                                                
                                                                    92 - KITCHEN EQUIPMENT
                                                                
                                                                    94 - KITCHEN ELECTRICAL
                                                                
                                                                    96 - KITCHEN PLUMBING
                                                                
                                                                    98 - LANDSCAPING &amp; IRRIGATION
                                                                
                                                                    100 - LAUNDRY APPLIANCES
                                                                
                                                                    102 - LAUNDRY ELECTRICAL
                                                                
                                                                    104 - LAUNDRY PLUMBING
                                                                
                                                                    106 - LIGHTING, DECORATIVE INTERIOR
                                                                
                                                                    108 - LIGHTING, DECORATIVE SITE
                                                                
                                                                    110 - LIGHTING, EXIT &amp; EMERGENCY
                                                                
                                                                    112 - LIGHTING, EXTERIOR DECORATIVE
                                                                
                                                                    114 - LIGHTING, PARKING LOT
                                                                
                                                                    116 - LOADING DOCK, SITE
                                                                
                                                                    118 - LOCKERS
                                                                
                                                                    120 - MAILBOXES
                                                                
                                                                    122 - MILLWORK
                                                                
                                                                    123 - MOVABLE PARTITIONS
                                                                
                                                                    124 - OIL SYSTEM PLUMBING
                                                                
                                                                    125 - PARKING TICKETING EQUIPMENT
                                                                
                                                                    126 - PARTITIONS, SPECIALTY
                                                                
                                                                    128 - PARTS DEPARTMENT ELECTRICAL
                                                                
                                                                    130 - PAVING, WALKS &amp; ROADS
                                                                
                                                                    132 - PHOTOVOLTAIC SYSTEM
                                                                
                                                                    134 - PIPE BOLLARDS, INTERIOR
                                                                
                                                                    136 - PIPE BOLLARDS, SITE
                                                                
                                                                    138 - PLUMBING, EQUIPMENT
                                                                
                                                                    140 - REFRIGERATION ELECTRIC
                                                                
                                                                    142 - REFRIGERATION EQUIPMENT
                                                                
                                                                    145 - SECURITY SYSTEMS
                                                                
                                                                    146 - SERVER ROOM HVAC
                                                                
                                                                    148 - SHOP CRANE
                                                                
                                                                    150 - SHOP ELECTRICAL
                                                                
                                                                    152 - SHOP EQUIPMENT STRUCTURE
                                                                
                                                                    154 - SHOP EXHAUST SYSTEM
                                                                
                                                                    156 - SHOP PLUMBING
                                                                
                                                                    158 - SIGNAGE, SITE
                                                                
                                                                    159 - SITE ATHLETICS COURTS
                                                                
                                                                    160 - SITE FURNISHING GAS CONNECTIONS
                                                                
                                                                    161 - SITE PORTE COCHERE
                                                                
                                                                    162 - SITE PRIVACY WALLS
                                                                
                                                                    163 - SITE RETAINING WALLS
                                                                
                                                                    164 - SITE SIGNAGE ELECTRICAL
                                                                
                                                                    166 - SITE SWIMMING POOLS
                                                                
                                                                    166.5 - SPECIALTY CONVEYING SYSTEMS
                                                                
                                                                    167 - SPECIALTY FIRE SUPPRESSION
                                                                
                                                                    168 - STORM DRAIN SYSTEM
                                                                
                                                                    170 - SURVEILLANCE SYSTEM
                                                                
                                                                    172 - SURVEILLANCE SYSTEM CONNECTIONS
                                                                
                                                                    174 - TELEPHONE SYSTEM
                                                                
                                                                    176 - TELEPHONE SYSTEM CONNECTIONS
                                                                
                                                                    178 - TELEVISION / CABLE CONNECTIONS
                                                                
                                                                    180 - TRASH ENCLOSURE
                                                                
                                                                    182 - VENDING EQUIPMENT ELECTRICAL
                                                                
                                                                    184 - WALL PROTECTION / CORNER GUARDS
                                                                
                                                                    186 - WASH BAY ELECTRICAL
                                                                
                                                                    188 - WASH BAY PLUMBING
                                                                
                                                                    190 - WINDOW COVERINGS
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        RS Means numbering
                                                        
                                                            Only by construction division
                                                            Both by construction divison and subgroup
                                                        
                                                    
                                                    
                                                        Item Construction Division /SubGroup Search
                                                        
                                                        
                                                            
                                                            
                                                                
                                                                    
                                                                        Items Search Result
                                                                        
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            *Construction Division
                                                            
                                                                
                                                                    Select
                                                                    
                                                                        1. General Requirements 
                                                                    
                                                                        2. Existing Conditions 
                                                                    
                                                                        3. Concrete 
                                                                    
                                                                        4. Masonry 
                                                                    
                                                                        5. Metals 
                                                                    
                                                                        6. Wood, Plastics, and Composites 
                                                                    
                                                                        7. Thermal and Moisture Protection 
                                                                    
                                                                        8. Openings 
                                                                    
                                                                        9. Finishes 
                                                                    
                                                                        10. Specialties 
                                                                    
                                                                        11. Equipment 
                                                                    
                                                                        12. Furnishings 
                                                                    
                                                                        13. Special Construction 
                                                                    
                                                                        14. Conveying Equipment 
                                                                    
                                                                        21. Fire Suppression 
                                                                    
                                                                        22. Plumbing 
                                                                    
                                                                        23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                                                    
                                                                        26. Electrical 
                                                                    
                                                                        27. Communications 
                                                                    
                                                                        28. Electronic Safety and Security 
                                                                    
                                                                        31. Earthwork 
                                                                    
                                                                        32. Exterior Improvements 
                                                                    
                                                                        33. Utilities 
                                                                    
                                                                        34. Transportation 
                                                                    
                                                                        35. Waterway and Marine Construction 
                                                                    
                                                                        41. Material Processing and Handling Equipment 
                                                                    
                                                                        44. Pollution and Waste Control Equipment 
                                                                    
                                                                        50. Landscaping Equipent 
                                                                    
                                                                        46. Water and Wastewater Equipment 
                                                                    
                                                                        48. Electrical Power Generation 
                                                                    
                                                                Select
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            *Item SubGroup
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            Backout this Takeoff from this Cost
                                                        
                                                        
                                                            Select
                                                            
                                                        
                                                    
                                                    
                                                
                                            
                                            
                                                
                                                    Save Custom Item &amp; Make Available for Future Use
                                                
                                                
                                                    Save and add another custom item
                                                    Save
                                                    Close
                                                    Reset
                                                    Show Others Fields
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            PurposeDescription
                                                            
                                                                
                                                                    Select
                                                                    
                                                                        A/V System
                                                                    
                                                                        A/V System Equipment
                                                                    
                                                                        A/V System Equipment Electric
                                                                    
                                                                        A/V System,
                                                                    
                                                                        Back Porch
                                                                    
                                                                        Backsplash Wall
                                                                    
                                                                        Balcony Exterior Doors,
                                                                    
                                                                        Balcony Floor Finish,
                                                                    
                                                                        Balcony Steel Railing,
                                                                    
                                                                        Balcony Wall Light,
                                                                    
                                                                        Base
                                                                    
                                                                        Base Molding,
                                                                    
                                                                        Basement Staircase,
                                                                    
                                                                        Bldg
                                                                    
                                                                        Bldg Basement
                                                                    
                                                                        Bldg Basement Concrete Floor,
                                                                    
                                                                        Bldg Basement Excavation, 
                                                                    
                                                                        Bldg Basement FDN Wall
                                                                    
                                                                        Bldg CC Column
                                                                    
                                                                        Bldg CC Elevated Floor Structure
                                                                    
                                                                        Bldg CC Footing
                                                                    
                                                                        Bldg CC Footings
                                                                    
                                                                        Bldg CC Slab
                                                                    
                                                                        Bldg CC Slab on Grade
                                                                    
                                                                        Bldg Column Fireproofing,
                                                                    
                                                                        Bldg Concrete Ext Wall
                                                                    
                                                                        Bldg Concrete Staircase,
                                                                    
                                                                        Bldg Elev Beam FDN
                                                                    
                                                                        Bldg Elev Concrete Floor
                                                                    
                                                                        Bldg Elev Floor Structure
                                                                    
                                                                        Bldg Elev Metal Joist Floor
                                                                    
                                                                        Bldg Elev Structural Steel Floor
                                                                    
                                                                        Bldg Elev Walkway
                                                                    
                                                                        Bldg Elev Wood Joist Floor
                                                                    
                                                                        Bldg Elevated Floor Structure
                                                                    
                                                                        Bldg Ext Brick Stucco Wall
                                                                    
                                                                        Bldg Ext Brick Veneer Wall
                                                                    
                                                                        Bldg Ext Brick Wall
                                                                    
                                                                        Bldg Ext CMU EIFS Wall
                                                                    
                                                                        Bldg Ext CMU Stucco Wall
                                                                    
                                                                        Bldg Ext CMU Wall
                                                                    
                                                                        Bldg Ext Column Covering
                                                                    
                                                                        Bldg Ext Concrete Wall Panels
                                                                    
                                                                        Bldg Ext Demountable Canopy
                                                                    
                                                                        Bldg Ext EIFS Wall
                                                                    
                                                                        Bldg Ext Glass Curtain Wall
                                                                    
                                                                        Bldg Ext Lighting
                                                                    
                                                                        Bldg Ext Metal Panel Curtain Wall
                                                                    
                                                                        Bldg Ext Metal Siding Wall
                                                                    
                                                                        Bldg Ext Metal Stud EIFS Wall
                                                                    
                                                                        Bldg Ext Metal Stud Fiber Cement Wall
                                                                    
                                                                        Bldg Ext Metal Stud Stucco Wall
                                                                    
                                                                        Bldg Ext Metal Stud Wall
                                                                    
                                                                        Bldg Ext Stone Veneer Wall
                                                                    
                                                                        Bldg Ext Tilt-Up CC Panel Wall
                                                                    
                                                                        Bldg Ext Vinyl Siding Wall
                                                                    
                                                                        Bldg Ext Wall
                                                                    
                                                                        Bldg Ext Wall Lighting
                                                                    
                                                                        Bldg Ext Wall Lighting Electrical
                                                                    
                                                                        Bldg Ext Wood Siding Wall
                                                                    
                                                                        Bldg Ext Wood Stud EIFS Wall
                                                                    
                                                                        Bldg Ext Wood Stud Fiber Cement Wall
                                                                    
                                                                        Bldg Ext Wood Stud Stucco Wall
                                                                    
                                                                        Bldg Ext Wood Stud Wall
                                                                    
                                                                        Bldg FDN
                                                                    
                                                                        Bldg Masonry Ext Wall
                                                                    
                                                                        Bldg Metal Ext Wall
                                                                    
                                                                        Bldg PEMB Structure
                                                                    
                                                                        Bldg Porte Cochere Brick Veneer Wall
                                                                    
                                                                        Bldg Porte Cochere Column Covering
                                                                    
                                                                        Bldg Porte Cochere Concrete Footing
                                                                    
                                                                        Bldg Porte Cochere EIFS Wall
                                                                    
                                                                        Bldg Porte Cochere Lighting
                                                                    
                                                                        Bldg Porte Cochere Roof Cover
                                                                    
                                                                        Bldg Porte Cochere Roof Drainage
                                                                    
                                                                        Bldg Porte Cochere Roof Structure
                                                                    
                                                                        Bldg Porte Cochere Steel Beams
                                                                    
                                                                        Bldg Porte Cochere Steel Column
                                                                    
                                                                        Bldg Porte Cochere Stone Veneer Wall
                                                                    
                                                                        Bldg Porte Cochere Wood Beams
                                                                    
                                                                        Bldg Porte Cochere Wood Columns
                                                                    
                                                                        Bldg Porte Cochere Wood Stud Stucco Wall
                                                                    
                                                                        Bldg Roof
                                                                    
                                                                        Bldg Roof Cover
                                                                    
                                                                        Bldg Roof Covering
                                                                    
                                                                        Bldg Roof Drainage
                                                                    
                                                                        Bldg Roof Structure
                                                                    
                                                                        Bldg Staircase,
                                                                    
                                                                        Bldg Steel Column
                                                                    
                                                                        Bldg Steel Railing,
                                                                    
                                                                        Bldg Steel Staircase,
                                                                    
                                                                        Bldg Strip Foundation
                                                                    
                                                                        Bldg Structural Steel Beam
                                                                    
                                                                        Bldg Unit Concrete Staircase,
                                                                    
                                                                        Bldg Unit Staircase,
                                                                    
                                                                        Bldg Unit Steel Staircase,
                                                                    
                                                                        Bldg Unit Wood Staircase,
                                                                    
                                                                        Bldg Walkway Concrete Finishing,
                                                                    
                                                                        Bldg Walkway Steel Railing,
                                                                    
                                                                        Bldg Wd Elevated Floor Structure
                                                                    
                                                                        Bldg Wood Column
                                                                    
                                                                        Bldg Wood Columns
                                                                    
                                                                        Bldg Wood Ext Wall
                                                                    
                                                                        Bldg Wood Staircase,
                                                                    
                                                                        Bldg-Mtd Lighting
                                                                    
                                                                        Break Area
                                                                    
                                                                        Break Area Base
                                                                    
                                                                        Break Area Base Cabs w/Ctr
                                                                    
                                                                        Break Area Base Cabs w/Ctr,
                                                                    
                                                                        Break Area Dbl Sink
                                                                    
                                                                        Break Area Dishwasher Electric,
                                                                    
                                                                        Break Area Elec Water Heater
                                                                    
                                                                        Break Area Equipment
                                                                    
                                                                        Break Area Equipment Electrical
                                                                    
                                                                        Break Area Gas Range/Oven
                                                                    
                                                                        Break Area Sink
                                                                    
                                                                        Break Area Wall
                                                                    
                                                                        Break Area Wall Cabinets
                                                                    
                                                                        Break Room Base Cabs w/Ctr
                                                                    
                                                                        Break Room Equipment
                                                                    
                                                                        Break Room Gas Range/Oven
                                                                    
                                                                        Break Room Sink
                                                                    
                                                                        Break Room Wall Cabinets
                                                                    
                                                                        Breakfast Buffet Base
                                                                    
                                                                        Breakfast Buffet Equipment
                                                                    
                                                                        Building Foundation Wall
                                                                    
                                                                        Building Spread Foundation
                                                                    
                                                                        Building-Mounted
                                                                    
                                                                        Building-Mounted Lighting
                                                                    
                                                                        Built-In Desk,
                                                                    
                                                                        Built-In Wardrobe,
                                                                    
                                                                        CCTV Electrical,
                                                                    
                                                                        Ceiling
                                                                    
                                                                        Ceiling Fan Electrical,
                                                                    
                                                                        Ceilings &amp; Partitions
                                                                    
                                                                        Ceramic Tile
                                                                    
                                                                        Chair Rail
                                                                    
                                                                        Checkout Counter,
                                                                    
                                                                        Chimney
                                                                    
                                                                        Closet
                                                                    
                                                                        Closet Door,
                                                                    
                                                                        Cold Storage Ceiling Unit Cooler,
                                                                    
                                                                        Cold Storage Condenser,
                                                                    
                                                                        Cold Storage Drain Piping,
                                                                    
                                                                        Cold Storage Electric,
                                                                    
                                                                        Cold Storage Refrigeration Piping,
                                                                    
                                                                        Cold Storage Remote Compressor,
                                                                    
                                                                        Cold Storage,
                                                                    
                                                                        Common
                                                                    
                                                                        Common Area
                                                                    
                                                                        Comms/Data Equipment
                                                                    
                                                                        Comms/Data Equipment Electric
                                                                    
                                                                        Compressed Air Equipment Electrical,
                                                                    
                                                                        Compressed Air Equipment,
                                                                    
                                                                        Compressed Air Piping,
                                                                    
                                                                        Computer Equipment
                                                                    
                                                                        Computer Equipment Electric
                                                                    
                                                                        Computer Room HVAC
                                                                    
                                                                        Concrete Topping Floor Finish
                                                                    
                                                                        Condiments &amp; Beverage Counter,
                                                                    
                                                                        Convenience Outlet,
                                                                    
                                                                        Crane Equipment Electrical,
                                                                    
                                                                        Crane Equipment,
                                                                    
                                                                        Crane Rail Steel Support,
                                                                    
                                                                        Crown Molding
                                                                    
                                                                        Data Center Access Flooring, 
                                                                    
                                                                        Data Center Electrical, 
                                                                    
                                                                        Data Center Fire Suppression System, 
                                                                    
                                                                        Data Center HVAC Electrical, 
                                                                    
                                                                        Data Center HVAC, 
                                                                    
                                                                        Data/Comms Equipment
                                                                    
                                                                        Decorative
                                                                    
                                                                        Decorative Pendant
                                                                    
                                                                        Decorative Pendant Light
                                                                    
                                                                        Decorative Pendant Lighting
                                                                    
                                                                        Demountable Mezzanine
                                                                    
                                                                        Demountable Partition, 
                                                                    
                                                                        Demountable PTAC HVAC System Electrical, 
                                                                    
                                                                        Demountable PTAC HVAC System, 
                                                                    
                                                                        Detached Garage Concrete Slab
                                                                    
                                                                        Detached Garage Ext Brick Veneer Wall
                                                                    
                                                                        Detached Garage Ext Metal Siding Wall
                                                                    
                                                                        Detached Garage Ext Stone Veneer Wall
                                                                    
                                                                        Detached Garage Ext Vinyl Siding Wall
                                                                    
                                                                        Detached Garage Ext Wall Lighting
                                                                    
                                                                        Detached Garage Ext Wall Lighting Electrical
                                                                    
                                                                        Detached Garage Ext Wood Siding Wall
                                                                    
                                                                        Detached Garage Ext Wood Stud Fiber Cement Wall
                                                                    
                                                                        Detached Garage Ext Wood Stud Stucco Wall
                                                                    
                                                                        Detached Garage Exterior Doors
                                                                    
                                                                        Detached Garage General Lighting
                                                                    
                                                                        Detached Garage Roof Cover
                                                                    
                                                                        Detached Garage Roof Drainage
                                                                    
                                                                        Detached Garage Roof Structure
                                                                    
                                                                        Detached Outdoor Deck,
                                                                    
                                                                        Dishwasher Drain,
                                                                    
                                                                        Dishwasher Electrical,
                                                                    
                                                                        Dishwasher Hood Electrical,
                                                                    
                                                                        Dishwasher Water Supply
                                                                    
                                                                        Elec Water Cooler
                                                                    
                                                                        Electric Water Cooler
                                                                    
                                                                        Electric Water Cooler Electrical
                                                                    
                                                                        Elev Wood Joist Floor,
                                                                    
                                                                        Elevated Floors
                                                                    
                                                                        Elevator
                                                                    
                                                                        Elevator Electrical
                                                                    
                                                                        Elevator Sump
                                                                    
                                                                        Elevator Sump Pump
                                                                    
                                                                        Elevator,
                                                                    
                                                                        Emergency
                                                                    
                                                                        Emergency Generator Equipment,
                                                                    
                                                                        Emergency Lighting
                                                                    
                                                                        Employee Break Area Base
                                                                    
                                                                        Employee Break Area Wall
                                                                    
                                                                        Employee Lounge
                                                                    
                                                                        Equip Elec Inc SVC
                                                                    
                                                                        Equipment Elec
                                                                    
                                                                        Equipment Electric
                                                                    
                                                                        Equipment Electric SVC
                                                                    
                                                                        Equipment Protection Bollards,
                                                                    
                                                                        Equipment Protection Guardrails,
                                                                    
                                                                        Exam Room Base Cabs w/Ctr
                                                                    
                                                                        Exam Room Equipment
                                                                    
                                                                        Exam Room Equipment Wash Sink
                                                                    
                                                                        Exam Room Plumbing
                                                                    
                                                                        Exam Room Sink
                                                                    
                                                                        Exam Room Wall Cabinets
                                                                    
                                                                        Exit
                                                                    
                                                                        Exit &amp; Emerg Lt Combo
                                                                    
                                                                        Exit Lighting
                                                                    
                                                                        Ext Garage
                                                                    
                                                                        Ext Stairs
                                                                    
                                                                        Exterior
                                                                    
                                                                        Exterior Canopy Can Lighting
                                                                    
                                                                        Exterior Concrete Stairs
                                                                    
                                                                        Exterior Doors
                                                                    
                                                                        Exterior Garage
                                                                    
                                                                        Exterior Staircase
                                                                    
                                                                        Exterior Storage Bldg Doors
                                                                    
                                                                        Exterior Storage Bldg Interior Partitions
                                                                    
                                                                        Exterior Storage Bldg Structure,
                                                                    
                                                                        Exterior Wall EIFS Coating Finish,
                                                                    
                                                                        Exterior Wall Paint Finish,
                                                                    
                                                                        Exterior Wall Stucco Finish,
                                                                    
                                                                        Exterior Window Treatment,
                                                                    
                                                                        Exterior Wood Stairs
                                                                    
                                                                        Eye Wash Stations
                                                                    
                                                                        FF &amp; E,
                                                                    
                                                                        Fire Alarm, 
                                                                    
                                                                        Fire Sprinkler System
                                                                    
                                                                        Fireplace
                                                                    
                                                                        Floating Floor - Common Area
                                                                    
                                                                        Food Prep Kitchen
                                                                    
                                                                        Food Prep Kitchen Base
                                                                    
                                                                        Food Prep Kitchen Equipment
                                                                    
                                                                        Food Prep Kitchen Sink
                                                                    
                                                                        Food Prep Kitchen Wall
                                                                    
                                                                        Front Desk w/Low Wall,
                                                                    
                                                                        Front Entry
                                                                    
                                                                        FRP Wall Covering,
                                                                    
                                                                        Fryer Electrical,
                                                                    
                                                                        Fume Hoods
                                                                    
                                                                        Gas Dryer Electrical,
                                                                    
                                                                        Gas Fryer
                                                                    
                                                                        Gas Griddle
                                                                    
                                                                        Gas Griddle &amp; Oven Combo
                                                                    
                                                                        Gas Range
                                                                    
                                                                        Gas Range &amp; Oven Combo
                                                                    
                                                                        Gas Range, Griddle, &amp; Oven Combo
                                                                    
                                                                        Gen Elec Inc SVC
                                                                    
                                                                        Gen Gas WH
                                                                    
                                                                        General
                                                                    
                                                                        General Bathtub
                                                                    
                                                                        General Domestic Water
                                                                    
                                                                        General Electric
                                                                    
                                                                        General Electric SVC
                                                                    
                                                                        General Electric Water Heater
                                                                    
                                                                        General Electric Water Htr
                                                                    
                                                                        General Equipment Piping
                                                                    
                                                                        General Gas
                                                                    
                                                                        General Gas Pipe,
                                                                    
                                                                        General Gas Water Heater
                                                                    
                                                                        General Lavatory
                                                                    
                                                                        General Lighting
                                                                    
                                                                        General Lighting Electrical,
                                                                    
                                                                        General San Sewer
                                                                    
                                                                        General Sanitary
                                                                    
                                                                        General Sanitary Sewer Pipe
                                                                    
                                                                        General Sanitary Sewer Vent Pipe,
                                                                    
                                                                        General Sanitary Sewer,
                                                                    
                                                                        General Sanitary Vent,
                                                                    
                                                                        General Shower
                                                                    
                                                                        General SVC Sink
                                                                    
                                                                        General Urinal
                                                                    
                                                                        General Water
                                                                    
                                                                        General Water Closet
                                                                    
                                                                        General Water Heater
                                                                    
                                                                        General Water Heater Gas Pipe,
                                                                    
                                                                        General Water Heater Water Pipe,
                                                                    
                                                                        General Water Pipe,
                                                                    
                                                                        General WC
                                                                    
                                                                        Griddle &amp; Oven Combo Electrical,
                                                                    
                                                                        Griddle Electrical,
                                                                    
                                                                        Guest Room Coffee Maker Electric
                                                                    
                                                                        Guest Room Electrical
                                                                    
                                                                        Guest Room Furniture,
                                                                    
                                                                        Guest Room Hair Dryer Electric
                                                                    
                                                                        Guest Room Microwave Electric
                                                                    
                                                                        Guest Room Microwave,
                                                                    
                                                                        Guest Room Mini Fridge Electric
                                                                    
                                                                        Guest Room Mini Fridge,
                                                                    
                                                                        Guest Room Refrigerator,
                                                                    
                                                                        Guest Room Television,
                                                                    
                                                                        Handicap Bathroom Accessories,
                                                                    
                                                                        Handicap Lavatory,
                                                                    
                                                                        Handicap Shower,
                                                                    
                                                                        Handsink Drain
                                                                    
                                                                        Handsink Water Supply
                                                                    
                                                                        Hostess Station Counter,
                                                                    
                                                                        Hot Tub Electrical
                                                                    
                                                                        HVAC
                                                                    
                                                                        HVAC AHU
                                                                    
                                                                        HVAC Drain Piping,
                                                                    
                                                                        HVAC Electric
                                                                    
                                                                        HVAC Electric Unit Heater
                                                                    
                                                                        HVAC Electrical
                                                                    
                                                                        HVAC Gas Furnace
                                                                    
                                                                        HVAC Gas Piping
                                                                    
                                                                        HVAC Gas Piping,
                                                                    
                                                                        HVAC RTU
                                                                    
                                                                        HVAC Split System
                                                                    
                                                                        HVAC Split System Elec Heater
                                                                    
                                                                        HVAC Split System Rmt Condenser
                                                                    
                                                                        HVAC SS
                                                                    
                                                                        HVAC SS AHU
                                                                    
                                                                        HVAC SS Rmt Cond
                                                                    
                                                                        Hydraulic Equipment
                                                                    
                                                                        Ice Cream Machine Electrical,
                                                                    
                                                                        Inc Gen Elec SVC
                                                                    
                                                                        Indoor
                                                                    
                                                                        Indoor Coiling Door
                                                                    
                                                                        Indoor Drinking Fountains,
                                                                    
                                                                        Indoor Fireplace,
                                                                    
                                                                        Indoor Hot Tub Electric Water Heater,
                                                                    
                                                                        Indoor Hot Tub Equipment Electrical,
                                                                    
                                                                        Indoor Hot Tub Gas Piping,
                                                                    
                                                                        Indoor Hot Tub Gas Water Heater,
                                                                    
                                                                        Indoor Hot Tub Water Piping,
                                                                    
                                                                        Indoor Jacuzzi
                                                                    
                                                                        Indoor Jacuzzi Equipment Electrical,
                                                                    
                                                                        Indoor Jacuzzi Gas Piping,
                                                                    
                                                                        Indoor Jacuzzi Heater,
                                                                    
                                                                        Indoor Jacuzzi Piping,
                                                                    
                                                                        Indoor Jacuzzi Water Supply Piping,
                                                                    
                                                                        Indoor Jacuzzi,
                                                                    
                                                                        Indoor Plant Wall Water
                                                                    
                                                                        Indoor Pool Deck
                                                                    
                                                                        Indoor Pool Dehumdifier,
                                                                    
                                                                        Indoor Pool Dehumidifer,
                                                                    
                                                                        Indoor Pool Equipment
                                                                    
                                                                        Indoor Pool Equipment Electrical,
                                                                    
                                                                        Indoor Pool Equipment Gas Piping,
                                                                    
                                                                        Indoor Pool Exhaust
                                                                    
                                                                        Indoor Pool Piping,
                                                                    
                                                                        Indoor Pool Water Supply Piping,
                                                                    
                                                                        Indoor Sauna Equipment
                                                                    
                                                                        Indoor Sauna Equipment Electrical
                                                                    
                                                                        Indoor Saunas
                                                                    
                                                                        Indoor Steam Room Door,
                                                                    
                                                                        Indoor Steam Room Drain Piping,
                                                                    
                                                                        Indoor Steam Room Equipment
                                                                    
                                                                        Indoor Steam Room Equipment Electrical
                                                                    
                                                                        Indoor Steam Room Gas Piping,
                                                                    
                                                                        Indoor Steam Room Piping
                                                                    
                                                                        Indoor Steam Room Water Supply Piping,
                                                                    
                                                                        Indoor Steam Rooms
                                                                    
                                                                        Indoor Vertical Lift Door
                                                                    
                                                                        Indoor Vertical Lift OHead
                                                                    
                                                                        Int Inc Equip Elec SVC
                                                                    
                                                                        Int Inc Gen Elec SVC
                                                                    
                                                                        Int of Ext Wall Drywall Sheathing
                                                                    
                                                                        Int of Exterior Wall
                                                                    
                                                                        Int Sec Coiling Door,
                                                                    
                                                                        Interior Closet
                                                                    
                                                                        Interior Door
                                                                    
                                                                        Interior Door,
                                                                    
                                                                        Interior Doors
                                                                    
                                                                        Interior Driveway
                                                                    
                                                                        Interior Partition
                                                                    
                                                                        Interior Partition,
                                                                    
                                                                        Interior Partitions
                                                                    
                                                                        Interior Perimeter Wall,
                                                                    
                                                                        Interior Rollup Door,
                                                                    
                                                                        Interior Staircase
                                                                    
                                                                        Interior Storage Doors
                                                                    
                                                                        Interior Storage Partitions
                                                                    
                                                                        Interior Window Treatment,
                                                                    
                                                                        Kitchen
                                                                    
                                                                        Kitchen 3-Comp Sink
                                                                    
                                                                        Kitchen Base Cabs
                                                                    
                                                                        Kitchen Base Cabs w/Ctr
                                                                    
                                                                        Kitchen Base Cabs,
                                                                    
                                                                        Kitchen Built-In
                                                                    
                                                                        Kitchen Counter
                                                                    
                                                                        Kitchen Counter,
                                                                    
                                                                        Kitchen Dishwasher Electric,
                                                                    
                                                                        Kitchen Elec Range/Oven
                                                                    
                                                                        Kitchen Equip Electrical
                                                                    
                                                                        Kitchen Equipment
                                                                    
                                                                        Kitchen Equipment Electric
                                                                    
                                                                        Kitchen Equipment Electrical
                                                                    
                                                                        Kitchen Exhaust
                                                                    
                                                                        Kitchen Gas Piping,
                                                                    
                                                                        Kitchen Gas Range
                                                                    
                                                                        Kitchen Gas Range/Oven
                                                                    
                                                                        Kitchen Gas Range/Oven,
                                                                    
                                                                        Kitchen Main Gas
                                                                    
                                                                        Kitchen Oven
                                                                    
                                                                        Kitchen Range Electrical
                                                                    
                                                                        Kitchen Range Electrical,
                                                                    
                                                                        Kitchen Sink
                                                                    
                                                                        Kitchen Sink,
                                                                    
                                                                        Kitchen Wall
                                                                    
                                                                        Kitchen Wall Cabs,
                                                                    
                                                                        Lab Sink
                                                                    
                                                                        Laboratory Base Cabs/w Ctrs,
                                                                    
                                                                        Laboratory Electric
                                                                    
                                                                        Laboratory Wall
                                                                    
                                                                        Laminate
                                                                    
                                                                        Laminate Flooring,
                                                                    
                                                                        Landscape
                                                                    
                                                                        Laundry
                                                                    
                                                                        Laundry Base
                                                                    
                                                                        Laundry Drain Piping,
                                                                    
                                                                        Laundry Dryer,
                                                                    
                                                                        Laundry Elec Dryer
                                                                    
                                                                        Laundry Electric Dryer
                                                                    
                                                                        Laundry Electric Water Heater
                                                                    
                                                                        Laundry Equipment
                                                                    
                                                                        Laundry Equipment Electrical
                                                                    
                                                                        Laundry Gas Dryer
                                                                    
                                                                        Laundry Gas Piping,
                                                                    
                                                                        Laundry Piping
                                                                    
                                                                        Laundry Sink
                                                                    
                                                                        Laundry Wall
                                                                    
                                                                        Laundry Washer
                                                                    
                                                                        Laundry Washer,
                                                                    
                                                                        Laundry Waste Pipe
                                                                    
                                                                        Laundry Water Pipe
                                                                    
                                                                        Laundry Water Piping,
                                                                    
                                                                        LVT
                                                                    
                                                                        Make-up Air Electrical,
                                                                    
                                                                        Manufacturing Equipment Electrical,
                                                                    
                                                                        Manufacturing Equipment Protection Bollards,
                                                                    
                                                                        Manufacturing Equipment Protection Guardrails,
                                                                    
                                                                        Manufacturing Equipment Water Piping,
                                                                    
                                                                        Medical Equipment Electric
                                                                    
                                                                        Medical Office Electric Water Heater
                                                                    
                                                                        Medical Office Fixture Waste
                                                                    
                                                                        Medical Office Fixture Water
                                                                    
                                                                        Medical Office Gas Water Heater
                                                                    
                                                                        Natural
                                                                    
                                                                        Nurse's Station w/Low Wall Ctr
                                                                    
                                                                        Nurses' Station Back Base Cabs w/Ctr
                                                                    
                                                                        Nurses' Station Desk w/Low Wall Ctr
                                                                    
                                                                        Nurses' Station Low Wall w.Ctr
                                                                    
                                                                        Nurses' Station Sink
                                                                    
                                                                        Nurses' Station Wall
                                                                    
                                                                        Nurses' Station Wall Cabinet
                                                                    
                                                                        Nurses' Station Wall Cabinets
                                                                    
                                                                        Office Base Cabinetry,
                                                                    
                                                                        Office Base Cabs w/Ctr
                                                                    
                                                                        Office Furniture Electric
                                                                    
                                                                        Office Wall
                                                                    
                                                                        Office Wall Cabinetry,
                                                                    
                                                                        Outdoor Deck,
                                                                    
                                                                        Parking Garage
                                                                    
                                                                        Parking Garage Bollards
                                                                    
                                                                        Parking Garage CC Column
                                                                    
                                                                        Parking Garage CC Footings
                                                                    
                                                                        Parking Garage CC Slab
                                                                    
                                                                        Parking Garage Drainage
                                                                    
                                                                        Parking Garage Elev Concrete Floor
                                                                    
                                                                        Parking Garage Elevator Electrical,
                                                                    
                                                                        Parking Garage Elevator,
                                                                    
                                                                        Parking Garage Ext Brick Wall
                                                                    
                                                                        Parking Garage Ext CMU Stucco Wall
                                                                    
                                                                        Parking Garage Ext CMU Wall
                                                                    
                                                                        Parking Garage Ext Glass Curtain Wall
                                                                    
                                                                        Parking Garage Ext Metal Panel Curtain Wall
                                                                    
                                                                        Parking Garage Ext Stone Veneer Wall
                                                                    
                                                                        Parking Garage Exterior Doors
                                                                    
                                                                        Parking Garage Lighting
                                                                    
                                                                        Parking Garage Sprinkler System
                                                                    
                                                                        Parking Garage Steel Railing
                                                                    
                                                                        Parking Garage Steel Railing,
                                                                    
                                                                        Pier and Beam Foundation CC Footing,
                                                                    
                                                                        Pier and Beam Foundation Excavation,
                                                                    
                                                                        Pier and Beam Foundation Wood Columns,
                                                                    
                                                                        Pier and Beam Wood Floor Construction,
                                                                    
                                                                        PoS Equipment
                                                                    
                                                                        Pot Sink Drain
                                                                    
                                                                        Pot Sink Water Supply
                                                                    
                                                                        Prescription Drug Refrigeration Equip
                                                                    
                                                                        Process Area Electric Water Heater,
                                                                    
                                                                        Process Area Gas Water Heater,
                                                                    
                                                                        Process Area Roof Exhaust Electrical,
                                                                    
                                                                        Process Area Roof Exhaust,
                                                                    
                                                                        Process Area Trench Drains,
                                                                    
                                                                        Process Area Wall Exhaust Electrical,
                                                                    
                                                                        Process Area Wall Exhaust,
                                                                    
                                                                        Process Area Water Heater Electrical,
                                                                    
                                                                        Process Area Water Heater Piping,
                                                                    
                                                                        PTAC HVAC Drain Piping,
                                                                    
                                                                        PVC Wall Covering,
                                                                    
                                                                        Range &amp; Oven Combo Electrical,
                                                                    
                                                                        Range Electrical,
                                                                    
                                                                        Range, Griddle, &amp; Oven Combo Electrical,
                                                                    
                                                                        Reach-in Refrigerator Electrical,
                                                                    
                                                                        Reception Back Base Cabinetry
                                                                    
                                                                        Reception Back Wall Cabinetry
                                                                    
                                                                        Reception Built-in Desk
                                                                    
                                                                        Reception Desk Glass Wall,
                                                                    
                                                                        Reception Desk w/Low Wall, 
                                                                    
                                                                        Refrigeration Equipment
                                                                    
                                                                        Refrigeration Room Electric,
                                                                    
                                                                        Removable Mezzanine
                                                                    
                                                                        Residential Unit
                                                                    
                                                                        Restroom
                                                                    
                                                                        Restroom Counter
                                                                    
                                                                        Restroom Countertop,
                                                                    
                                                                        Restroom Electric,
                                                                    
                                                                        Restroom Exhaust
                                                                    
                                                                        Restroom Hand Dryer
                                                                    
                                                                        Restroom Lavatory,
                                                                    
                                                                        Restroom Mirror
                                                                    
                                                                        Restroom Urinal,
                                                                    
                                                                        Restroom Vanity
                                                                    
                                                                        Restroom Vanity Base Cabinets,
                                                                    
                                                                        Restroom Vanity Base Cabs w/Ctr
                                                                    
                                                                        Restroom Vanity Countertop,
                                                                    
                                                                        Restroom Water Closet,
                                                                    
                                                                        Roof Drain
                                                                    
                                                                        Roof-Mtd Photovoltaic System
                                                                    
                                                                        Sanitary
                                                                    
                                                                        Security
                                                                    
                                                                        Server Room HVAC
                                                                    
                                                                        Sheet Vinyl,
                                                                    
                                                                        Shower Enclosures,
                                                                    
                                                                        Site
                                                                    
                                                                        Site Asphalt
                                                                    
                                                                        Site Asphalt Berm
                                                                    
                                                                        Site Asphalt Driveway
                                                                    
                                                                        Site Asphalt Paving
                                                                    
                                                                        Site Barbecue Grille
                                                                    
                                                                        Site Basketball
                                                                    
                                                                        Site Bollards,
                                                                    
                                                                        Site Brick Pavers
                                                                    
                                                                        Site Carports,
                                                                    
                                                                        Site CC Curb &amp; Gutter
                                                                    
                                                                        Site CC Driveway
                                                                    
                                                                        Site CC Patio
                                                                    
                                                                        Site CC Sidewalk
                                                                    
                                                                        Site CC Slab on Grade
                                                                    
                                                                        Site CC Truck Ramp
                                                                    
                                                                        Site Chain Link
                                                                    
                                                                        Site Concrete Curb
                                                                    
                                                                        Site Concrete Curb &amp; Gutter
                                                                    
                                                                        Site Concrete Patio
                                                                    
                                                                        Site Concrete Paving
                                                                    
                                                                        Site Concrete Sidewalk
                                                                    
                                                                        Site Concrete Walkways
                                                                    
                                                                        Site Decorative Pole Lighting
                                                                    
                                                                        Site Driveway Pavers
                                                                    
                                                                        Site Dumpster Enclosure
                                                                    
                                                                        Site Equipment Protection Bollards,
                                                                    
                                                                        Site Fencing
                                                                    
                                                                        Site Fire Hydrant
                                                                    
                                                                        Site Flagpoles,
                                                                    
                                                                        Site Gravel Driveway
                                                                    
                                                                        Site Gravel Walkways
                                                                    
                                                                        Site Guardrail,
                                                                    
                                                                        Site Handrails,
                                                                    
                                                                        Site Hot Tub
                                                                    
                                                                        Site Hot Tub Equipment
                                                                    
                                                                        Site Hot Tub Equipment Electrical
                                                                    
                                                                        Site Hot Tub Equipment Piping
                                                                    
                                                                        Site Hot Tub Pad
                                                                    
                                                                        Site Incoming Electric SVC
                                                                    
                                                                        Site Incoming Fire Protection SVC
                                                                    
                                                                        Site Incoming N Gas SVC
                                                                    
                                                                        Site Incoming San Sewer SVC
                                                                    
                                                                        Site Incoming Water SVC
                                                                    
                                                                        Site Irrigation
                                                                    
                                                                        Site Jacuzzi
                                                                    
                                                                        Site Jacuzzi Heater
                                                                    
                                                                        Site Jacuzzi,
                                                                    
                                                                        Site Landscape
                                                                    
                                                                        Site Landscaping
                                                                    
                                                                        Site Parking Curb,
                                                                    
                                                                        Site Parking Curbs,
                                                                    
                                                                        Site Parking Signs,
                                                                    
                                                                        Site Patio Fencing,
                                                                    
                                                                        Site Patio Pavers
                                                                    
                                                                        Site Planter,
                                                                    
                                                                        Site Pole Lighting
                                                                    
                                                                        Site Porte Cochere Brick Veneer Finish
                                                                    
                                                                        Site Porte Cochere Column Covering
                                                                    
                                                                        Site Porte Cochere Concrete Footing
                                                                    
                                                                        Site Porte Cochere EIFS Finish
                                                                    
                                                                        Site Porte Cochere Lighting
                                                                    
                                                                        Site Porte Cochere Roof Cover
                                                                    
                                                                        Site Porte Cochere Roof Drainage
                                                                    
                                                                        Site Porte Cochere Roof Structure
                                                                    
                                                                        Site Porte Cochere Steel Beams
                                                                    
                                                                        Site Porte Cochere Steel Columns
                                                                    
                                                                        Site Porte Cochere Stone Veneer Finish
                                                                    
                                                                        Site Porte Cochere Stucco Finish
                                                                    
                                                                        Site Porte Cochere Wood Beams
                                                                    
                                                                        Site Porte Cochere Wood Columns
                                                                    
                                                                        Site Privacy Wall
                                                                    
                                                                        Site Reinf CC Paving
                                                                    
                                                                        Site Retaining Wall
                                                                    
                                                                        Site Slate Pavers
                                                                    
                                                                        Site Storm Drainage
                                                                    
                                                                        Site Storm Water Detention Area
                                                                    
                                                                        Site Swimming Pool
                                                                    
                                                                        Site Swimming Pool Deck
                                                                    
                                                                        Site Swimming Pool Enclosure
                                                                    
                                                                        Site Swimming Pool Equipment
                                                                    
                                                                        Site Swimming Pool Equipment Electrical
                                                                    
                                                                        Site Swimming Pool Equipment Piping
                                                                    
                                                                        Site Walkway Pavers
                                                                    
                                                                        Site Water Line
                                                                    
                                                                        Soiled Utility Sink
                                                                    
                                                                        Sprinkler System
                                                                    
                                                                        Stone Tile Finishing,
                                                                    
                                                                        Storage Base Cabs w/Ctrs,
                                                                    
                                                                        Storage Wall
                                                                    
                                                                        Storefront Doors,
                                                                    
                                                                        Storefront Windows,
                                                                    
                                                                        Tall Storage Cabinets
                                                                    
                                                                        Tenant Kitchen Spaces
                                                                    
                                                                        Tenant Kitchen Spaces Gas WH
                                                                    
                                                                        Tenant Space Finishes
                                                                    
                                                                        Tenant Spaces Food Prep 3-Comp Sink
                                                                    
                                                                        Terrace-Mounted Light Bollard
                                                                    
                                                                        Vanity Base Cabinet,
                                                                    
                                                                        Vanity Cabinet Countertop,
                                                                    
                                                                        Vanity Countertop,
                                                                    
                                                                        Vanity Sink,
                                                                    
                                                                        VCT
                                                                    
                                                                        VCT,
                                                                    
                                                                        Vent Kit Electrical,
                                                                    
                                                                        Vinyl Sheet Flooring,
                                                                    
                                                                        Vinyl Wall Covering,
                                                                    
                                                                        Walk-in Cooler Drain,
                                                                    
                                                                        Walk-in Cooler Electric,
                                                                    
                                                                        Walk-in Cooler,
                                                                    
                                                                        Walk-in Freezer Drain,
                                                                    
                                                                        Walk-in Freezer Electric,
                                                                    
                                                                        Walk-in Freezer,
                                                                    
                                                                        Walk-in Refrigerator Drain,
                                                                    
                                                                        Walk-in Refrigerator Electrical,
                                                                    
                                                                        Walk-in Refrigerator,
                                                                    
                                                                        Wall
                                                                    
                                                                        Wall Paneling,
                                                                    
                                                                        Wall Tiles,
                                                                    
                                                                        Wallpaper Wall Covering,
                                                                    
                                                                        Wallpaper,
                                                                    
                                                                        Water Distribution Piping,
                                                                    
                                                                        Water Heater Electrical
                                                                    
                                                                        Water Heater Gas Piping,
                                                                    
                                                                        Water Heater Piping
                                                                    
                                                                        Water Heater Piping,
                                                                    
                                                                        Water Heater Water Piping,
                                                                    
                                                                        Wet Bar
                                                                    
                                                                        Wet Bar Sink
                                                                    
                                                                        Window Shades
                                                                    
                                                                        Window Treatment
                                                                    
                                                                        Windows
                                                                    
                                                                        Wood Flooring
                                                                    
                                                                        Wood Flooring,
                                                                    
                                                                        Wood Wall Paneling,
                                                                    
                                                                        Work Room Base Cabs w/Ctr
                                                                    
                                                                        Work Room Wall Cabinets
                                                                    
                                                                        X-Ray Equipment
                                                                    
                                                                        X-Ray Equipment Electrical
                                                                    
                                                                        X-Ray Shielded Doors
                                                                    
                                                                        X-Ray Shielded Partition
                                                                    
                                                                        X-Ray Shielded Window
                                                                    
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            Labour Cost
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Material Cost
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Equipment Cost
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            CF2
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            CF
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            CF4
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                        
                                        
                                        
                                    
                                
                            
                        
                        
                    
                
            
        
    


    .multiselect-container>li>a>label>input[type=radio]{
        position: relative;
        top: 4px;
    }
    .dropdown-menu>li {
        height: 45px;
    }



    

    $(&quot;[name='rsmeans_numbering']&quot;).on(&quot;change&quot;, function(){
        if($(this).val() == '1'){
            $(&quot;#id_item_sub_group_no_error_parent_div&quot;).hide();
            $(&quot;#custom_item_search_box&quot;).hide();
        }
        else{
            $(&quot;#id_item_sub_group_no_error_parent_div&quot;).show();
            $(&quot;#custom_item_search_box&quot;).show();
        }
    });

    

    $(document).ready(function(){
        $(&quot;#id_item_group_no&quot;).select2({
             dropdownParent: $('#add_item_assembly'),
             width: 'resolve'
        });

        $(&quot;#id_item_group_no&quot;).on('select2:open', function(){
            select2_open = true;
        });

        $(&quot;#id_item_group_no&quot;).on('select2:close', function(){
            select2_open = false;
        });

        

    })

    
        $(document).on('change', '#id_purpose_description', function(){
            var check = $('#hidden_value_for_check').val();
            var value = $(this).val();
            console.log(check, 'here')
            if (check &amp;&amp; value &amp;&amp; value != &quot;Others&quot;){
                $.ajax({
                    type: 'GET',
                    url: '/project/map-recovery-period-and-group/',
                    data:{
                        'concat_id': value,
                        'property_id': '3338',
                    },
                    success: function(response){

                        if(response){

                            $('#id_recovery_period option[value=&quot;'+response.recovery_code+'&quot;]').prop('selected','selected');
                            
                                $('#id_asset_class2').val(response.asset_id);
                            
                        }
                    }

                });
            }

        });

    


    
    
        
            
                ×
                  Basic KeyBoard Shortcuts
            
            
                
                    
                        
                            Keyboard
                            Functionality
                        
                    
                    
                        
                            Shift + i
                            
                                Show/Hide Project Information
                            
                        
                        
                            Shift + w
                            
                                Show Analysis Window
                            
                        
                        
                            Shift + z
                            
                                Add Items/Assemblies
                            
                        
                        
                            Shift + x
                            
                                Add Custom Item
                            
                        
                        
                            Shift + r
                            
                                Add Custom Assembly
                            
                        
                        
                            Shift + s
                            
                                Show Advanced Bulk Select
                            
                        
                        
                            Shift + m
                            
                                Show Advanced Modify Modal
                            
                        
                        
                            Shift + g
                            
                                Close Dialouge Box
                            
                        
                        
                            Shift + e
                            
                                Show/hide the related items
                            
                        
                        
                            Shift + t
                            
                                Toggle Costs.
                            
                        
                        
                            Shift + f
                            
                                Toggle Additional Record Information.
                            
                        
                        
                            Shift + v
                            
                                Toggle between all views.
                            
                        
                        
                            Shift + a
                            
                                Select all Items
                            
                        
                        
                            Esc
                            
                                Deselect all Items
                            
                        
                        
                            Shift + d
                            
                                Delete selected Items
                            
                        
                        
                            Shift + c
                            
                                Update selected Purpose Description Prefix
                            
                        
                        
                            Shift + l
                            
                                Update selected Location
                            
                        
                        
                            Shift + y
                            
                                Update selected Asset year
                            
                        
                        
                            Shift + h
                            
                                Open keyboard shortcuts.
                            
                        
                    
                
            
            
                Close
            
        
    


    
    
        
            
                ×
                  Warning
            
            
                
                    You have selected a template that is of one Property Type, the PPA Property Type, and you have selected to use a different Property Type for this property, the New Construction Property Type.
                    
                    Note: This is allowed but you should understand that there will be some added complexity involved by making this selection.
                    
                    PPA Jobs are built without Direct Cost Data from the Contractor(s) who built the property and, as a result, these jobs do not include any Direct Costs (entered as &quot;Custom Items&quot; within the application).
                    
                    The complexity introduced here, by using a template with this Property Type for this property, is that no Direct Contract Costs from this property currently exist inside of this Worksheet but there are many items and assemblies entered as takeoff. In order for the application to work optimally, all of these items and assemblies must be associated as being &quot;backed out of&quot; a particular Takeoff Cost.
                    
                    First, you will need to input the Direct Costs associated with your property and designate one or more of them to be Takeoff Costs from which the value of the items and assemblies will be deducted. Once the Takeoff Costs have been entered, you can begin to associate each of the items and assemblies with a specific Takeoff Cost from within each item's/assembly's details dialogue by double left-clicking on each record one at a time.
                    
                    Note: Once you have inputted the Takeoff Costs and designated a Takeoff Cost for each cost for each item/assembly, you should now see that the Initial Total Cost and the Active Total Cost are identical. If these numbers are not the same then make sure you have accounted for every single item and assembly being associated with a takeoff cost.
                
            
        
    


    
    
        
            
                ×
                  Warning
            
            
                
                    You have selected a template that is of one Property Type, the New Construction Property Type and you have selected to use a different Property Type with this property, the PPA Property Type.
                    
                    Note: This is allowed but you should understand that there will be some added complexity involved by making this selection.
                    
                    New Construction Property Type Jobs are built around having entered Direct Costs (entered as &quot;Custom Items&quot; within the application) from the Contractor(s) who built the property and then backing out any takeoff of items and assemblies from a specifically designated subset of these Direct Costs, known as Takeoff Costs within the application. This is done so that the value of these takeoffs will be auto-subtracted from these Takeoff Costs and the value of the entire property will remain unchanged throughout the job after all Direct Costs are entered.
                    
                    The complexity introduced here, by using a template with this Property Type for this property, is that there will exist Direct Contract Costs from an entirely unrelated property inside this Worksheet and some of these, those designated as Takeoff Costs, will be associated directly with the items and assemblies entered as takeoff for the property.
                    
                    The easiest solution to get rid of the complexity is just to select and delete all of the Direct Costs in the Worksheet. This is done by selecting each of the Custom Items (by Shift or CTRL left-click) and then using the Delete Command (Shift-D) or by using the Delete Button found in the Additional Controls menu at the top of the Dashboard screen once inside the job.
                    
                    All that will be left in this case will be items and assemblies that were entered from the RS Means database. Of course, if you need to cull additional items or assemblies at this point you can do so by using the same mechanism described above.
                
            
        
    


    
    
        
            
                ×
                Confirm Delete
            
            
                Are you sure you want to delete  records?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                Confirm Delete
            
            
                Are you sure you want to delete  records?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                 Bulk Select
            
            
                
                    

                        
                            Purpose Description Prefix
                            
                                Select
                                None
                                
                            
                        

                        
                            Location
                            
                                Select
                                
                            
                        

                        

                        
                            Recovery Period
                            
                                Select
                                
                                
                            
                        

                        
                            Asset Class
                            
                                Select
                                None
                                
                            
                        


                    
                    

                        
                            Construction Division
                            
                                Select
                                None
                                
                                    1. General Requirements 
                                
                                    2. Existing Conditions 
                                
                                    3. Concrete 
                                
                                    4. Masonry 
                                
                                    5. Metals 
                                
                                    6. Wood, Plastics, and Composites 
                                
                                    7. Thermal and Moisture Protection 
                                
                                    8. Openings 
                                
                                    9. Finishes 
                                
                                    10. Specialties 
                                
                                    11. Equipment 
                                
                                    12. Furnishings 
                                
                                    13. Special Construction 
                                
                                    14. Conveying Equipment 
                                
                                    21. Fire Suppression 
                                
                                    22. Plumbing 
                                
                                    23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                
                                    26. Electrical 
                                
                                    27. Communications 
                                
                                    28. Electronic Safety and Security 
                                
                                    31. Earthwork 
                                
                                    32. Exterior Improvements 
                                
                                    33. Utilities 
                                
                                    34. Transportation 
                                
                                    35. Waterway and Marine Construction 
                                
                                    41. Material Processing and Handling Equipment 
                                
                                    44. Pollution and Waste Control Equipment 
                                
                                    50. Landscaping Equipent 
                                
                                    46. Water and Wastewater Equipment 
                                
                                    48. Electrical Power Generation 
                                
                            
                        

                        
                            Record Type
                            
                                Select
                                Custom Item
                                Individual Item
                                Assemblies
                                Takeoff Cost
                            
                        

                        
                            Takeoff Cost Association
                            
                                Select
                                None
                                
                            
                        

                        
                            Units
                            
                                Select
                                None
                                
                            
                        

                        
                            Cost Source
                            
                                Select
                                None
                                
                                    Contractor Cost
                                
                                    Client Cost
                                
                                    RS Means
                                
                                    Site Contractor Cost
                                
                                    Site Indirect
                                
                                    Client Cost - Expenses
                                
                            
                        

                    
                    

                        
                            Dynamic Attribute Keys
                            
                                Select
                                None
                                
                            
                        

                        
                            Modules
                            
                                Select
                                None
                                
                            
                        

                        
                            Quantity
                            
                        

                        
                            Keyword Search (Ex: &quot;Sprinkler&quot;, &quot;~System&quot;)
                            
                        
                    
                
                
                    
                        Note: The Keyword Search tool above will not search inside of Purpose Description Prefixes. To change Purpose Description Prefixes, choose one from that Dropdown Menu
                    

                    
                        
                            
                            Select Inverse
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
        
    
        
            
            
                
                    ×
                     Filter Items
                
                
                    
                        

                            
                                Purpose Description Prefix
                                
                                    Select
                                    None
                                    
                                
                            

                            
                            

                            
                                Location
                                
                                    Select
                                    
                                
                            

                            

                            
                                Recovery Period
                                
                                    Select
                                    
                                    
                                
                            

                            
                                Asset Class
                                
                                    Select
                                    None
                                    
                                
                            

                        
                        

                            
                                Construction Division
                                
                                    Select
                                    None
                                    
                                        1. General Requirements 
                                    
                                        2. Existing Conditions 
                                    
                                        3. Concrete 
                                    
                                        4. Masonry 
                                    
                                        5. Metals 
                                    
                                        6. Wood, Plastics, and Composites 
                                    
                                        7. Thermal and Moisture Protection 
                                    
                                        8. Openings 
                                    
                                        9. Finishes 
                                    
                                        10. Specialties 
                                    
                                        11. Equipment 
                                    
                                        12. Furnishings 
                                    
                                        13. Special Construction 
                                    
                                        14. Conveying Equipment 
                                    
                                        21. Fire Suppression 
                                    
                                        22. Plumbing 
                                    
                                        23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                    
                                        26. Electrical 
                                    
                                        27. Communications 
                                    
                                        28. Electronic Safety and Security 
                                    
                                        31. Earthwork 
                                    
                                        32. Exterior Improvements 
                                    
                                        33. Utilities 
                                    
                                        34. Transportation 
                                    
                                        35. Waterway and Marine Construction 
                                    
                                        41. Material Processing and Handling Equipment 
                                    
                                        44. Pollution and Waste Control Equipment 
                                    
                                        50. Landscaping Equipent 
                                    
                                        46. Water and Wastewater Equipment 
                                    
                                        48. Electrical Power Generation 
                                    
                                
                            

                            
                                Record Type
                                
                                    Select
                                    Custom Item
                                    Individual Item
                                    Assemblies
                                    Takeoff Cost
                                
                            

                            
                                Takeoff Cost Association
                                
                                    Select
                                    None
                                    
                                
                            

                            
                                Units
                                
                                    Select
                                    None
                                    
                                
                            

                            
                                Cost Source
                                
                                    Select
                                    None
                                    
                                        Contractor Cost
                                    
                                        Client Cost
                                    
                                        RS Means
                                    
                                        Site Contractor Cost
                                    
                                        Site Indirect
                                    
                                        Client Cost - Expenses
                                    
                                
                            

                        
                        

                            
                                Modules
                                
                                    Select
                                    None
                                    
                                
                            

                            
                                Quantity
                                
                            

                            
                                Keyword Search (Ex: &quot;Sprinkler&quot;, &quot;~System&quot;)
                                
                            
                        
                    
                    
                        
                            Note: The Keyword Search tool above will not search inside of Purpose Description Prefixes. To change Purpose Description Prefixes, choose one from that Dropdown Menu
                        

                        
                            
                                
                                Select Inverse
                            
                        
                    
                
                
                    Modify
                    Reset
                    Filter
                    Cancel
                
            
        
    


    
    
    
        
            
                ×
                 Bulk Select By Location
            
            
                
                    
                        Location
                    
                    
                        
                            Select
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    



    
    
        
            
                ×
                 Bulk Select By Recovery Period
            
            
                
                    
                        Recovery Period
                    
                    
                        
                            Select
                            
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
        
            
                ×
                 Bulk Select By Asset Class
            
            
                
                    
                        Asset Class
                    
                    
                        
                            Select
                            None
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
        
            
                ×
                 Bulk Select By Construction Division
            
            
                
                    
                        Construction Division
                    
                    
                        
                            Select
                            None
                            
                                1. General Requirements 
                            
                                2. Existing Conditions 
                            
                                3. Concrete 
                            
                                4. Masonry 
                            
                                5. Metals 
                            
                                6. Wood, Plastics, and Composites 
                            
                                7. Thermal and Moisture Protection 
                            
                                8. Openings 
                            
                                9. Finishes 
                            
                                10. Specialties 
                            
                                11. Equipment 
                            
                                12. Furnishings 
                            
                                13. Special Construction 
                            
                                14. Conveying Equipment 
                            
                                21. Fire Suppression 
                            
                                22. Plumbing 
                            
                                23. Heating, Ventilating, and Air Conditioning (HVAC) 
                            
                                26. Electrical 
                            
                                27. Communications 
                            
                                28. Electronic Safety and Security 
                            
                                31. Earthwork 
                            
                                32. Exterior Improvements 
                            
                                33. Utilities 
                            
                                34. Transportation 
                            
                                35. Waterway and Marine Construction 
                            
                                41. Material Processing and Handling Equipment 
                            
                                44. Pollution and Waste Control Equipment 
                            
                                50. Landscaping Equipent 
                            
                                46. Water and Wastewater Equipment 
                            
                                48. Electrical Power Generation 
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
        
            
                ×
                Associate Asset Class
            
            
                
                
                    
                        
                            
                                Choose Depreciation
                            
                        
                        
                            
                            
                            
                        
                    
                
                
                
                
                    Apply
                    Cancel
                
            
        
    




        $(document).on('click', '#associate_asset_class_btn', function(e){
            e.preventDefault();
            var valid = true;
            if(!$('#select_depreciation').val()){
                valid = false;
                $('.custom_error').text('This field is required');
            }

            if(valid){
                var row_id = $('#depreciation_row_id').val();
                $.ajax({
                    type: 'POST',
                    url: '/depreciation/associate-asset-class/',
                    data: {
                        'type': 'assign_asset_class',
                        'csrfmiddlewaretoken': 'h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5',
                        'depreciation_id': $('#select_depreciation').val(),
                        'recovery_period': $('#recovery_period_id').val()
                    },
                    success: function(response){
                        if (response){
                            $('#'+row_id+' .asset_class').multiselect('clearSelection');
                            $('#'+row_id+' .asset_class').multiselect('select', response.asset_class_ids)
                            recovery_period_manager.update_all_recovery_period($('#'+row_id+' .asset_class'));
                            $('#associate_asset_class').modal('hide');
                        }
                    }
                });
            }

        });







    
    var read_only_view = false;
    
    var notification_modal = false;

    $(&quot;#view_mode_form&quot;).on(&quot;submit&quot;, function(e){
        window.scrollTo(0, 0);
    });

    
    if(JSON.parse(localStorage.getItem(&quot;show_modification_message&quot;))){
        $(&quot;#show-modification-info&quot;).show();
    }
    
    if(JSON.parse(localStorage.getItem(&quot;open_filter_tool&quot;))){
        $(&quot;#nonselected&quot;).modal(&quot;show&quot;);
        localStorage.setItem(&quot;open_filter_tool&quot;, &quot;false&quot;)
    }
    
    

    $(&quot;#id_cus_additem_form #id_unit_cost, #id_cus_additem_form #id_cus_quantity&quot;).on('keyup', function(){
        if ($(&quot;#id_cus_additem_form #id_unit_cost&quot;).val()){
            unit_cost = parseFloat($(&quot;#id_cus_additem_form #id_unit_cost&quot;).val().replace('$', '').replace(/,/g, ''));
        }
        else{
            unit_cost = 0;
        }
        if($(&quot;#id_cus_additem_form #id_cus_quantity&quot;).val()){
            quantity =  parseFloat($(&quot;#id_cus_additem_form #id_cus_quantity&quot;).val().replace(/,/, ''));
        }
        else{
            quantity = 1;
        }
        total_cost = (unit_cost * quantity).toFixed(2);
        $(&quot;#id_cus_additem_form #id_total_cost&quot;).val(total_cost.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
    })

    $(&quot;#id_cus_additem_form #id_takeoff_cost&quot;).on(&quot;change&quot;, function(){
        if ($(&quot;#id_takeoff_cost&quot;).prop(&quot;checked&quot;) == true){
            $(&quot;#id_cus_additem_form #id_backoff_cost_error_parent_div&quot;).hide();
        }
        else{
            $(&quot;#id_cus_additem_form #id_backoff_cost_error_parent_div&quot;).show();
        }
    })

    var isAssemblyTitlesLock = false;
    var filter_active = false
    var customAssembliesList = []
    
    var customAssembliesQuantity = {}
    
    var filter_data = {}
    
    var assembliesChilds = []
    var assemblies2Childs = []
    var parentList = []
    var isAdditonalControl = false
    var isFullScreen = false
    var isToggleCost = false
    var storage = window.localStorage;
    var showModal = storage.getItem('showModal');
    var showAddItemModal = storage.getItem('showAddItemModal');
    var customAssemblyGroup = storage.getItem('search_assembly_group_no');
    var TAKEOFF_COST_OPTIONS = &quot;&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost1' selected='selected'>TC #1&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost2' >TC #2&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost3' >TC #3&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost4' >TC #4&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost5' >TC #5&lt;/option>&quot;
    
    var constructionNumber = storage.getItem('constructionNumber');
    var assemblyNumber = storage.getItem('assemblyNumber');
    var toggleCost = storage.getItem('toggleCost');
    if (toggleCost == null){
        toggleCost = false;
    }
    var view = storage.getItem('view');

    function clone_property_confirm(id){
        // $(&quot;#id_delete_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_property(&quot; + id + &quot;)&quot;);
        $('#id_clone_existing_property').val(id)
        $(&quot;#id_clone_property_modal&quot;).modal(&quot;show&quot;);
    }

    // function reset filtered items
    function resetFilteredItems(){
        event.preventDefault();
        $('.filter_item_select').val('');
        $('.filter_item_select_inverse').prop('checked', false);
        $('#reset_filter_id').val('true')
        $('#id_processing').show();
        $('#id_item_filter_form').submit()
    }

    // queryFilteredItems
    function queryFilteredItems(){
        event.preventDefault();

        var purpose_description = $('#id_filter_purpose_description').val()
        var location = $('#id_new_filter_location').val()
        var building = $('#id_new_filter_building').val()
        var asset_class = $('#id_new_filter_asset_class').val()
        var recovery_period = $('#id_new_filter_recovery_period').val()
        var quantity = $('#id_new_filter_quantity').val()
        var division = $('#id_new_filter_division').val()
        var record_type = $('#id_new_filter_type').val()
        var takeoff_cost_association = $('#id_new_filter_takeoffcost').val()
        var units = $('#id_new_filter_unit').val()
        var cost_source = $('#id_new_filter_costsource').val()
        var modules = $('#id_new_filter_module').val();
        var keyword = $('#id_new_filter_keyword').val();

        var select_inverse = $('#filter_select_inverse_check').is(':checked');

        data = {
            &quot;purpose_description&quot;: purpose_description,
            &quot;location&quot;: location,
            &quot;building&quot;: building,
            &quot;asset_class&quot;: asset_class,
            &quot;recovery_period&quot;: recovery_period,
            &quot;quantity&quot;: quantity,
            &quot;division&quot;: division,
            &quot;record_type&quot;: record_type,
            &quot;takeoff_cost_association&quot;: takeoff_cost_association,
            &quot;units&quot;: units,
            &quot;cost_source&quot;: cost_source,
            &quot;modules&quot;: modules,
            &quot;keyword&quot;: keyword,
            &quot;select_inverse&quot;: select_inverse,
        }

        $('#reset_filter_id').val('')

        $('#json_data_filter').val(JSON.stringify(data))
        $('#id_processing').show()

        $('#id_item_filter_form').submit()

    }

    function uploadCostFile(id, type){
        // $('#id_processing').show();

        $('#id_processing').show();

        var formdata = new FormData();

        formdata.append('files', $('#id_import_csv_cost').prop('files')[0]);
        formdata.append('type', type);
        formdata.append('csrfmiddlewaretoken', 'h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5');

        $.ajax({
            type: 'POST',
            url: '/project/import-cost-setup/'+id+'/',
            data: formdata,
            processData: false,
            contentType: false,
            success: function(response){

                console.log(response.cost_setup)
                for (setup in response.cost_setup){
                    form.addRow('', response.cost_setup[setup]);
                }
                $('#id_processing').hide();
                $('#importCostSetup').modal('hide');

            }
        });

    }

    function uploadFile(id, building_id){

        $('#id_processing').show();

        var formdata = new FormData();

        formdata.append('files', $('#id_import_file').prop('files')[0]);
        formdata.append('building_id', $('#import_attribute_id').val());
        formdata.append('csrfmiddlewaretoken', 'h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5');

        $.ajax({
            type: 'POST',
            url: '/project/import-dynamic-attributes/'+id+'/',
            data: formdata,
            processData: false,
            contentType: false,
            success: function(response){
                if(response.invalid_file){
                    $(&quot;#id_error_import_message&quot;).html(response.error_message);
                    $(&quot;#id_error_import_modal&quot;).modal(&quot;show&quot;);
                    return
                }

                for(var key in response.attributes){
                    if (response.attributes[key].type == &quot;number&quot;){

                        if ((response.attributes[key].value != null) &amp;&amp; (response.attributes[key].value.toString().trim() != '')){
                            $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').val(response.attributes[key].value);
                        }

                    }else if (response.attributes[key].type == &quot;option&quot;){
                        $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').val(response.attributes[key].value);

                    }else if (response.attributes[key].type == &quot;radio&quot;){
                        if (response.attributes[key].value == 'Yes'){

                            $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;][value=&quot;1&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').prop(&quot;checked&quot;, true);

                        }else{
                            $('input[value=&quot;1&quot;] .dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;][value=&quot;0&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').prop(&quot;checked&quot;, true);

                        }
                    }else{
                        $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').val(response.attributes[key].value);
                    }

                }
                if(!response.correct_data){
                    $(&quot;#id_error_import_message&quot;).html(response.error_message);
                    $(&quot;#id_error_import_modal&quot;).modal(&quot;show&quot;);
                }

                $('.dynamic_attributes[data-building_id=&quot;'+response.building_id+'&quot;]').last().trigger('change');
                $('#id_processing').hide();
                $('#importDynamicAttribute').modal('hide');
            }
        })
    }

    function show_hide_child_heirarchy() {
        ''
            if($('.id_assemb_items').is(':visible')) {
                $(&quot;.id_assemb_items_parent, .id_assemb_items&quot;).hide();
            } else {
                $(&quot;.id_assemb_items&quot;).show();
            }
        ''
    }

    $(&quot;.alert&quot;).delay(5000).slideUp(300);
    var state = $('input[name=state_status]:checked').val();

    var is_modal_open = false;

    

    $('#add_item_assembly').on('shown.bs.modal', function (e) {
        is_modal_open = true;
        $(&quot;#id_item&quot;).focus();
    })

    $('#add_item_assembly').on('shown.bs.modal', function (e) {
        if(!showAddItemModal){
            $(&quot;input[id='hidden_value_for_check']&quot;).val('')
            trigger_events($('#id_additem_form'));
            calculate_global_pricing();
            trigger_events($('#id_cus_additem_form'));
        }
    })

    $(&quot;#deleteZeroQuatityModal&quot;).on('show.bs.modal', function(e){
        deSelectFilteredItems();
        selectedIndex = [];

        var quantity = &quot;0&quot;;

        var target = $('[data-id]');
        target.each(function() {
            var item_quantity = $(this).siblings(&quot;.quantity&quot;).text();
            var isSelected = true;

            if(quantity &amp;&amp; parseFloat(quantity) != parseFloat(item_quantity)){
                isSelected = false;
            }

            if(isSelected){
                $(this).addClass('selected');
                selectedIndex.push($(this).attr('data-id'));
            }
        });
        $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);
        var records = selectedIndex.length;

        if(!records){
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#nozeroquantity&quot;).modal(&quot;show&quot;);
        }
        else{
            $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
        }
    })

    $(&quot;#deleteZeroQuatityModal&quot;).on('hidden.bs.modal', function(e){
        deSelectFilteredItems();
    })

    $('#summarysection').on('hidden.bs.modal', function () {
        is_modal_open = false;
    })

    $('#add_item_assembly').on('hidden.bs.modal', function () {
        is_modal_open = false;
        storage.setItem('showModal', '');
        storage.setItem('showAddItemModal', '');
        storage.setItem('constructionNumber', '');
        storage.setItem('assemblyNumber', '');
        const_div = '';
        assmb_grp = '';

        if($(&quot;#normal&quot;).hasClass('active')){
            save_input($('#id_additem_form'));
        }else{
            save_input($('#id_cus_additem_form'));
        }

        setTimeout(function(){
            window.location.reload();
        });
    })

    $(&quot;#id_custom_assembly_form&quot;).submit(function(e){
        e.preventDefault();
        return false;
    });

    function bindModalEvent() {
        $('.modal').each(function(){
            if($(this).attr('id') != 'add_item_assembly'){
                $(this).on('hidden.bs.modal', function (e) {
                    is_modal_open = false;
                });
            }
        });
    }

    bindModalEvent();

    var INACTIVE_TIME = new Date().getTime();
    $(document.body).bind(&quot;click&quot;, function (e) {
        INACTIVE_TIME = new Date().getTime();
    });

    $(document.body).bind(&quot;keypress&quot;, function () {
        INACTIVE_TIME = new Date().getTime();
    });

    $(&quot;#id_item_group_no&quot;).change(function () {
        var value = this.value;
        if (value == &quot;&quot; ){
            $(&quot;#id_item_sub_group_no&quot;).remove();
            $(&quot;#id_disabledgroup&quot;).show();
            $(&quot;#append_item_sub_group_no&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no&quot;>\
                        &lt;option value=&quot;&quot;>Select&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_disabledgroup&quot;).hide();
                    $(&quot;#append_item_sub_group_no&quot;).show();
                    $(&quot;#append_item_sub_group_no&quot;).html(select);
                    $(document).trigger(&quot;item_subgroup&quot;);
                }
            });
        }
    });

    $(&quot;#id_squeezetype&quot;).change(function(){
        val = $(this).val();
        if(val == &quot;squeeze_by_cons_div&quot;){
            $(&quot;#id_squeeze_cons_div_no_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_cons_div_no_input&quot;).hide();
        }

        if(val == &quot;squeeze_by_uniformat&quot;){
            $(&quot;#id_squeeze_uniformat_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_uniformat_input&quot;).hide();
        }

        if(val == &quot;squeeze_by_recovery_period&quot;){
            $(&quot;#id_squeeze_recovery_period_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_recovery_period_input&quot;).hide();
        }
        if(val == &quot;squeeze_by_location&quot;){
            $(&quot;#id_squeeze_location_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_location_input&quot;).hide();
        }
        if(val == &quot;squeeze_entire_takeoffsheet&quot;){
            $(&quot;#id_squeeze_entire_sheet_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_entire_sheet_input&quot;).hide();
        }if(val == &quot;squeeze_by_custom_backoff_cost&quot;){
            $(&quot;#id_squeeze_by_custom_backoff_cost_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_by_custom_backoff_cost_input&quot;).hide();
        }
    });

    $('input[type=radio][name=search_option]').change(function() {
        search_option = $(&quot;input[name='&quot; + $(this).attr('name') + &quot;']:checked&quot;).val();
        $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;);
        getCuratedItems(async=true);
    });

    $('input[type=radio][name=search_option_1]').change(function() {
        search_option = $(&quot;input[name='&quot; + $(this).attr('name') + &quot;']:checked&quot;).val();
        $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;);
        getCuratedItems2(async=true);
    });

    search_option = &quot;curated&quot;
    state_status = &quot;default&quot;
    const_div = &quot;&quot;
    assmb_grp = &quot;&quot;
    sub_group = &quot;&quot;
    assemb_sub_group = &quot;&quot;

    $('#id_construction_division').change(function() {
        const_div = $(this).val();
        $(&quot;#id_item_sub_group_search_div&quot;).hide();
        sub_group = &quot;&quot;
        getCuratedItems(async=true);
        var value = $(this).find(':selected').attr('data-division-value');
        if (value == undefined ){
            $(&quot;#id_item_sub_group_search_div&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no_items&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_item_sub_group_search_div&quot;).show();
                    $(&quot;#id_item_sub_group_search_div&quot;).html(select);
                    $('#id_item_sub_group_no_items').change(function() {
                        sub_group = $(this).val();
                        getCuratedItems(async=true);
                    });
                    $(document).trigger('subconstruction_loaded')
                }else{
                    $(&quot;#id_item_sub_group_search_div&quot;).hide();
                }
                }
            });
        }
    });


    $('#id_search_assembly_group_no').change(function() {
        assmb_grp = $(this).val();
        getCuratedItems(async=true);

        $(&quot;#id_assembly_sub_group_search_div&quot;).hide();

        assemb_sub_group = &quot;&quot;

        var value = $(this).val();
        if (value == undefined || value == ''){
            $(&quot;#id_assembly_sub_group_search_div&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-assembly-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;assembly_sub_group_no&quot; id=&quot;id_assembly_sub_group_no_items&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                        for(var i=0; i&lt;response.length; i++){
                            var option = &quot;&lt;option value=&quot;+response[i].id+&quot;>&quot;+ response[i].id + &quot;. &quot; + response[i].title + &quot;&lt;/option>&quot;;
                            $(select).append(option)
                        }
                        $(&quot;#id_assembly_sub_group_search_div&quot;).show();
                        $(&quot;#id_assembly_sub_group_search_div&quot;).html(select);

                        $('#id_assembly_sub_group_no_items').change(function() {
                            assemb_sub_group = $(this).val();
                            getCuratedItems();
                        });
                    }else{
                        $(&quot;#id_assembly_sub_group_search_div&quot;).hide();
                    }
                }
            });
        }
    });

    $('#id_construction_division_1').change(function() {
        $(&quot;#id_search_items&quot;).val(&quot;&quot;);
        const_div = $(this).val();
        $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
        sub_group = &quot;&quot;
        getCuratedItems2();
        var value = $(this).find(':selected').attr('data-division-value');
        if (value == undefined ){
            $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no_items_1&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_item_sub_group_search_div_1&quot;).show();
                    $(&quot;#id_item_sub_group_search_div_1&quot;).html(select);
                    $('#id_item_sub_group_no_items_1').change(function() {
                        sub_group = $(this).val();
                        getCuratedItems2();
                    });
                    $(document).trigger('subconstruction_loaded1')
                }
                else{
                    $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
                }
                }
            });
        }
    });

    $('#id_construction_division_2').change(function() {
        const_div = $(this).val();
        $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
        sub_group = &quot;&quot;

        var value = $(this).find(':selected').attr('data-division-value');
        if (value == undefined ){
            $(&quot;#id_item_sub_group_search_div_2&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no_items_2&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_item_sub_group_search_div_2&quot;).show();
                    $(&quot;#id_item_sub_group_search_div_2&quot;).html(select);
                }
            });
        }
    });

    $('#id_search_assembly_group_no_1').change(function() {
        $(&quot;#id_search_items&quot;).val(&quot;&quot;);
        assmb_grp = $(this).val();
        getCuratedItems2();

        $(&quot;#id_assembly_sub_group_search_div_1&quot;).hide();

        assemb_sub_group = &quot;&quot;

        var value = $(this).val();
        if (value == undefined || value == ''){
            $(&quot;#id_assembly_sub_group_search_div_1&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-assembly-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;assembly_sub_group_no&quot; id=&quot;id_assembly_sub_group_no_items_1&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                        for(var i=0; i&lt;response.length; i++){
                            var option = &quot;&lt;option value=&quot;+response[i].id+&quot;>&quot;+ response[i].id + &quot;. &quot; + response[i].title + &quot;&lt;/option>&quot;;
                            $(select).append(option)
                        }
                        $(&quot;#id_assembly_sub_group_search_div_1&quot;).show();
                        $(&quot;#id_assembly_sub_group_search_div_1&quot;).html(select);

                        $('#id_assembly_sub_group_no_items_1').change(function() {
                            assemb_sub_group = $(this).val();
                            getCuratedItems2();
                        });
                    }else{
                        $(&quot;#id_assembly_sub_group_search_div_1&quot;).hide();
                    }
                }
            });
        }
    });

    

    

    window.onbeforeunload = function(event) {
        window.localStorage.setItem('Xoffset', window.pageXOffset)
        window.localStorage.setItem('Yoffset', window.pageYOffset)
    };

    $(document).ready(function () {
        // $('#id_filter_purpose_description').val(filter_data.purpose_description)
        // $('#id_new_filter_location').val(filter_data.location)
        // $('#id_new_filter_building').val(filter_data.building)
        // $('#id_new_filter_asset_class').val(filter_data.asset_class)
        // $('#id_new_filter_recovery_period').val(filter_data.recovery_period)
        // $('#id_new_filter_quantity').val(filter_data.quantity)
        // $('#id_new_filter_division').val(filter_data.division)
        // $('#id_new_filter_type').val(filter_data.record_type)
        // $('#id_new_filter_takeoffcost').val(filter_data.takeoff_cost_association)
        // $('#id_new_filter_unit').val(filter_data.units)
        // $('#id_new_filter_costsource').val(filter_data.cost_source)
        // $('#id_new_filter_module').val(filter_data.module);
        // $('#id_new_filter_keyword').val(filter_data.keyword);

        if(filter_data){
            if(filter_data.purpose_description || filter_data.location || filter_data.building || filter_data.asset_class || filter_data.recovery_period || filter_data.quantity || filter_data.division || filter_data.record_type || filter_data.takeoff_cost_association || filter_data.units || filter_data.cost_source || filter_data.module || filter_data.keyword){
                filter_active = true
            }else{
                filter_active = false
            }
        }

        if (filter_active){
            $('#filter-circle').show();
            $('#filter-color-change').css('color', 'green');
            $('#filter-color-change').css('background', 'red');
            $('#filter-change-name').html('Active Filter');
        }

        $('td.quantity, #id_update_quantity').on('keydown', function(e){
            var code = e.keyCode

            if(code == 189){
                return false;
            }
        });
        // $('.building_types').multiselect({
        //     buttonWidth: '100%',
        //     nonSelectedText: 'Select',
        //     enableFiltering: false,
        //     maxHeight: 300,
        // });

        var newItemId = window.localStorage.getItem('NewItemId');

        if (newItemId){
            window.localStorage.setItem('NewItemId', '');
            //~ $(&quot;html, body&quot;).animate({
                //~ scrollTop: $('#item-id-'+newItemId).offset().top - $(&quot;body > nav&quot;).outerHeight() - $(&quot;#content > table:nth-child(2) > thead&quot;).height(),
            //~ }, 500);
            window.setTimeout(function() {
                window.scrollTo(window.localStorage.getItem('Xoffset'), $('#item-id-'+newItemId).offset().top - $(&quot;body > nav&quot;).outerHeight() - $(&quot;#content > table:nth-child(2) > thead&quot;).height() - 40)
            }, 500)
        }else{
            
                //window.setTimeout(function() {
                //    window.scrollTo(window.localStorage.getItem('Xoffset'), window.localStorage.getItem('Yoffset'))
                //}, 600)
            
        }

        var tmp = $('input#id_tie_to_no_apply_squeeze').val();

        var tmp3 = Number(tmp).toLocaleString(&quot;en-US&quot;, {style:&quot;currency&quot;, currency:&quot;USD&quot;, minimumFractionDigits: 2});

        if (tmp3 == 'NaN' | tmp3 == '$NaN' | tmp3 == '') {
            $('input#id_tie_to_no_apply_squeeze').val(tmp)
        } else {
            $('input#id_tie_to_no_apply_squeeze').val(tmp3);
        }

        $('input#id_tie_to_no_apply_squeeze').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }
        });

        $('#id_tie_to_no_apply_squeeze').bind('paste', function () {
            var self = this;
            setTimeout(function () {
                if (!/\d*(\.\d{1,2})+$/.test($(self).val())) $(self).val('');
            }, 0);
        });

        $('#id_custom_assembly_name').keypress(function(event) {
            $(&quot;#id_name_error&quot;).hide();
        });

        $('input[type=radio][name=level]').change(function() {
            $(&quot;#id_level_error&quot;).hide();
        });

        $('input#id_tie_to_no_apply_squeeze').keyup(function(e) {
            if ($(this).val().replace('$','').replace(',','').length &lt;= 13){
                $('#apply_squeeze_button_action').prop('disabled', false);
                $('.apply_squeeze_error').html('')
                if ($(this).val() == &quot;0.0&quot;){
                    $(this).val() = &quot;&quot;
                }else{
                    var $this = $(this);
                    var num = $this.val().replace(/,/g, '');
                    $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
                }
            }else{
                $('.apply_squeeze_error').html('Invalid Input')
                $('#apply_squeeze_button_action').prop('disabled', true);

            }
        });

        var ParentId = window.localStorage.getItem('ParentId')
        if (ParentId) {
            $(&quot;.&quot; + ParentId).show()
            window.localStorage.removeItem('ParentId')
        }

        $(&quot;#id_toggle_nav_action&quot;).click(function(){
            $(&quot;#id_nav_action&quot;).slideToggle();
            isAdditonalControl = !isAdditonalControl;
            UpdateTableHeader();
        });

        $( function() {
            $(&quot;#id_is_curated&quot;).click(function(){
                var curated_item_id = $(&quot;#item_hidden_id&quot;).val();
                var curated_item_status = $(&quot;#id_is_curated&quot;).is(':checked');
                $.ajax({
                    url: &quot;/project/change-item-curated/&quot;,
                    type: &quot;GET&quot;,
                    data: {
                        'curated_item_id': curated_item_id,
                        'curated_item_status': curated_item_status
                    },
                    success: function(response){
                        if (response.status) {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated&quot;).prop('checked', true);
                            } else {
                                $(&quot;#id_is_curated&quot;).prop('checked', false);
                            }
                        } else {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated&quot;).prop('checked', false);
                            } else {
                                $(&quot;#id_is_curated&quot;).prop('checked', true);
                            }
                        }
                    }
                })
            })
        });

        $( function() {
            $(&quot;#id_is_curated_1&quot;).click(function(){
                var curated_item_id = $(&quot;#item_hidden_id_1&quot;).val();
                var curated_item_status = $(&quot;#id_is_curated_1&quot;).is(':checked');
                $.ajax({
                    url: &quot;/project/change-item-curated/&quot;,
                    type: &quot;GET&quot;,
                    data: {
                        'curated_item_id': curated_item_id,
                        'curated_item_status': curated_item_status
                    },
                    success: function(response){
                        if (response.status) {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', true);
                            } else {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', false);
                            }
                        } else {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', false);
                            } else {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', true);
                            }
                        }
                    }
                })
            })
        });

        $(function() {
            $('.multiselect-ui').multiselect({
                includeSelectAllOption: true
            });
        });

        add_border();

        $( function() {
            $('.two-digits').keyup(function(){
                if($(this).val().indexOf('.') != -1){
                    if($(this).val().split(&quot;.&quot;)[1].length > 2) {
                        if( isNaN( parseFloat( this.value ) ) ) return;
                        this.value = parseFloat(this.value).toFixed(2);
                    }
                 }
                 return this;
            });
        });

        
            if (showAddItemModal === 'true'){
                var new_assembly = storage.getItem('NewCustomAssembly', '');
                is_modal_open = true;
                $(&quot;#add_item_assembly&quot;).modal('show');
                $('#id_search_assembly_group_no').val(customAssemblyGroup);
                assmb_grp = customAssemblyGroup;
                storage.setItem('showAddItemModal', '');
                $('#id_item').val(new_assembly);
                $.when(trigger_events($('#id_additem_form')), calculate_global_pricing(), getCuratedItems(async=false, result_hide=true)).then(
                    function(){
                        selectTypeAheadItem($('#'+new_assembly)[0]);
                        $('.typeahead-item').find('.item--suggestion').hide();

                        $('#id_quantity').focus();
                    }
                );
            }
            else if(showModal === &quot;true&quot;){
                is_modal_open = true;
                $(&quot;#add_item_assembly&quot;).modal('show');
                $('.nav-tabs a[href=&quot;#custom&quot;]').tab('show');
                $(&quot;#id_description&quot;).focus();
                $(&quot;#id_description&quot;).val(&quot;&quot;);
            }
            else if(showModal === &quot;algorithm&quot;){
                is_modal_open = true;
                $(&quot;#add_item_assembly&quot;).modal('show');
                $('.nav-tabs a[href=&quot;#algorithm-tab&quot;]').tab('show');
                $(&quot;#id_algorithm&quot;).focus();
            }
            else if(showModal === &quot;item&quot;){
                var same_item = storage.getItem('SameItemAgain', '')
                is_modal_open = true;
                $(&quot;#add_item_assembly&quot;).modal('show');
                $('.nav-tabs a[href=&quot;#normal&quot;]').tab('show');
                $(&quot;#id_description&quot;).val(&quot;&quot;);

                if (same_item == &quot;&quot;){
                    $(&quot;#id_construction_division&quot;).val(constructionNumber);
                    $(&quot;#id_search_assembly_group_no&quot;).val(assemblyNumber);

                    $(&quot;#id_construction_division_1&quot;).val(constructionNumber);
                    $(&quot;#id_search_assembly_group_no_1&quot;).val(assemblyNumber);
                }
                const_div = constructionNumber
                assmb_grp = assemblyNumber

                if (same_item != &quot;&quot;){
                    $('#id_item').val(same_item);
                    $.when(trigger_events($('#id_additem_form')), calculate_global_pricing(), getCuratedItems(async=false, result_hide=true)).then(
                        function(){
                            selectTypeAheadItem($('#'+same_item)[0]);
                            $('.typeahead-item').find('.item--suggestion').hide();
                            $('#id_quantity').focus();
                        }
                    );
                }else if (constructionNumber !== &quot;&quot; || assemblyNumber !== &quot;&quot;){
                    getCuratedItems();
                }
                $(&quot;#id_description&quot;).focus();
            }

            $(document).keypress(function(e){
                var code = e.shiftKey &amp;&amp; e.which;
                var key = e.which;
                if (read_only_view_enabled){
                    e.preventDefault();
                    e.stopPropagation();
                    return
                }
                if(key == 13){
                    if ($(&quot;input&quot;).is(':focus')){
                        e.preventDefault();
                    };
                }
                if(code == 90){
                    if (!is_modal_open){
                        is_modal_open = true;
                        e.preventDefault();
                        $('.modal').modal('hide');
                        $(&quot;#add_item_assembly&quot;).modal('show');
                        $('.nav-tabs a[href=&quot;#normal&quot;]').tab('show');
                        $(&quot;#id_item&quot;).focus();
                    }
                }else if(code == 70){
                    if (!is_modal_open){
                        toggle_record_info();
                    }
                }else if(code == 77){
                    if (!is_modal_open){
                        $(&quot;#bulk_tool_select&quot;).modal(&quot;show&quot;);
                    }
                }else if(code == 83){
                    if (!is_modal_open){
                        $(&quot;#itemSelect&quot;).modal(&quot;show&quot;);
                    }
                }
                else if(code == 71){
                    if (!is_modal_open){
                        is_modal_open = false;
                        $('.modal').modal('hide');
                    }
                }else if(code == 72){
                    if (!is_modal_open){
                        is_modal_open = false;
                        $('.modal').modal('hide');
                        $(&quot;#helpbox&quot;).modal(&quot;show&quot;);
                    }
                }
                else if(code == 88){
                    if (!is_modal_open){
                        is_modal_open = true;
                        e.preventDefault();
                        // $('.modal').modal('hide');
                        $(&quot;#add_item_assembly&quot;).modal('show');
                        $('.nav-tabs a[href=&quot;#custom&quot;]').tab('show');
                        $(&quot;#id_description&quot;).focus();
                    }
                }else if(code == 82){
                    if (!is_modal_open){
                        e.preventDefault();
                        window.location.href=&quot;/custom-assembly-wizard/?next=/project/take-offsheet/3338/&quot;;
                    }
                }
                 else if(code == 84){
                    if (!is_modal_open){
                        toggle_cost();
                    }
                } else if(code == 67){
                    if (!is_modal_open){
                        is_modal_open = false;
                        if($('td').hasClass('selected')){
                            $('.modal').modal('hide');
                            $.ajax({
                                url: &quot;/project/check-custom-for-purpose_description/&quot;,
                                type: &quot;GET&quot;,
                                data: {
                                    'selected_ids' : selectedIndex.join('_'),
                                },
                                success: function(response){
                                    is_modal_open = true
                                    $(&quot;#update_purpose_description&quot;).modal(&quot;show&quot;)
                                    if (response.is_custom_present) {
                                        $(&quot;#cus_concat_warning&quot;).css('display', 'block');
                                        if (response.only_custom_present) {
                                            $(&quot;#id-concat-update&quot;).css('display', 'none');
                                        } else {
                                            $(&quot;#id-concat-update&quot;).css('display', 'block');
                                        }
                                    } else {
                                        $(&quot;#cus_concat_warning&quot;).css('display', 'none');
                                        $(&quot;#id-concat-update&quot;).css('display', 'block');
                                    }
                                }
                            });
                        }else{
                            nonSelect()
                        }
                    }
                } else if(code == 68 ){
                    if (!is_modal_open){
                        if($('td').hasClass('selected')){
                            var records = selectedIndex.length;
                            $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
                            $('#deleteConfirmModal').modal(&quot;show&quot;);
                        }else{
                            nonSelect()
                        }
                    }
                } else if(code == 32){
                    if (!is_modal_open){
                        $(&quot;#id_view_by_construction&quot;).attr('checked', 'checked');
                        $(&quot;#view_mode_form&quot;).submit();
                    }

                } else if(code == 76){
                    if (!is_modal_open){
                        if($('td').hasClass('selected')){
                            is_modal_open = true;
                            $('.modal').modal('hide');
                            $(&quot;#update_location&quot;).modal(&quot;show&quot;)
                        }else{
                            nonSelect()
                        }
                    }
                } else if(code == 86){
                    if(!is_modal_open){
                        var nextRadio = $(&quot;input[name=view_mode]:checked&quot;).closest('.radio').next('.radio');
                        if(nextRadio.length) {
                            nextRadio.find('[name=&quot;view_mode&quot;]').prop('checked', true)
                        } else {
                            var firstRadio = $(&quot;input[name=view_mode]:checked&quot;).closest('form#view_mode_form').find('[name=&quot;view_mode&quot;]').first()
                            firstRadio.prop('checked', true)
                        }
                        $(&quot;#view_mode_form&quot;).submit();
                    }

                } else if(code == 89){
                    if (!is_modal_open){
                        if($('td').hasClass('selected')){
                            is_modal_open = true;
                            $('.modal').modal('hide');
                            $(&quot;#update_asset_year&quot;).modal(&quot;show&quot;)
                        }else{
                            nonSelect()
                        }
                    }
                } else if(code == 87 ){
                    if (!is_modal_open){
                        is_modal_open = false;
                        $('.modal').modal('hide');
                        estimate_result(3338);
                    }
                } else if(code == 73){
                    if (!is_modal_open){
                        var selectedEffect = &quot;drop&quot;;
                        $(&quot;#id_info_detail_short&quot;).toggle();
                        $(&quot;#id_info_detail&quot;).toggle( selectedEffect, 500, callback);
                    }
                } else if(code == 65){
                    if (!is_modal_open){
                        var selectAllItems = true;
                        var target = $('[' + $(&quot;#id_select_item&quot;).attr('data-select') + ']');

                        if (selectAllItems) {
                            target.each(function() {
                                if (!$(this).hasClass('selected')) {
                                    $(this).addClass('selected');
                                    selectedIndex.push($(this).attr('data-id'));
                                }
                            })
                            $(this).find('[toggle-class]').removeClass('fa-circle-o').addClass('fa-check-circle-o');
                            $(&quot;.pre-dis&quot;).removeAttr(&quot;disabled&quot;);
                            $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);
                        } else {
                            target.each(function() {
                                var id = $(this).attr('data-id');
                                $(this).removeClass('selected');
                                selectedIndex.splice(selectedIndex.indexOf(id), 1);
                            })
                            $(this).find('[toggle-class]').removeClass('fa-check-circle-o').addClass('fa-circle-o');
                            $(&quot;.pre-dis&quot;).attr(&quot;disabled&quot;, 'true');
                            $(&quot;input[name='id_lists']&quot; ).val(&quot;&quot;);
                        }
                        selectAllItems = !selectAllItems;
                    }
                }
            });
        

        $(document).keyup(function(e) {
            if (e.keyCode === 27){
                deSelectFilteredItems();
            }
        });

        function nonSelect(){
            $('.modal').modal('hide');
            is_modal_open = true;
            $(&quot;#nonselected&quot;).modal('show');
        }

        
            property_id = '3338'
        

        $(&quot;#id_cus_item_property&quot;).val(property_id);
        $(&quot;#id_quantity_property&quot;).val(property_id);
        $(&quot;#id_con_property&quot;).val(property_id);
        $(&quot;#id_loc_property&quot;).val(property_id);

        $('#id_quantity, #id_cus_quantity, #item_details #id_edit_quantity, #id_aitem_edit_quantity, #id_unit_cost').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }
        });

        $('#update_purpose_description').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_con&quot;).focus();
        });

        $('#update_filtered_purpose_description').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_filter_purpose&quot;).focus();
        });

        $('#update_asset_year').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_asset&quot;).focus();
        });

        $('#update_recovery_period').on('shown.bs.modal', function() {
            is_modal_open = true
            var records = selectedIndex.length;

            $('#id_records_count_asset').html('&lt;b>'+records+'&lt;/b>');

            $(&quot;#id_edit_bulk_class&quot;).focus();
        });

        $('#update_cost_source').on('shown.bs.modal', function() {
            is_modal_open = true
            var records = selectedIndex.length;

            $('#id_records_count_cost_source').html('&lt;b>'+records+'&lt;/b>');

        });

        $('#update_location').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_loc&quot;).focus();
        });

        $('#id_quantity').keyup(function(e) {
            var $this = $(this);
            var num = $this.val().replace(/,/g, '');
            $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        });

        $('#id_total_cost').keyup(function(e) {
            var $this = $(this);
            var num = $this.val().replace(/,/g, '');
            $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        });

        $('#id_unit_cost').keyup(function(e) {
            var $this = $(this);
            var num = $this.val().replace(/,/g, '');
            $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        });

        $('#id_total_cost').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }else{
                var velement = event.target || event.srcElement
                var fstpart_val = velement.value;
                var parts = velement.value.split('.');
                if (parts.length == 2 &amp;&amp; parts[1].length >= 2) event.preventDefault();
            }
        });

        $('#id_unit_cost').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }else{
                var velement = event.target || event.srcElement
                var fstpart_val = velement.value;
                var parts = velement.value.split('.');
                if (parts.length == 2 &amp;&amp; parts[1].length >= 2) event.preventDefault();
            }
        });

        $('[data-toggle=&quot;tooltip&quot;]').tooltip();

        $('#id_location').change(function() {
            if (this.value == &quot;Others&quot;){
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    $(&quot;#id_building_error_parent_div&quot;).show();
                }
                $(&quot;#id_extra_location_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_location_error_parent_div&quot;).hide();
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                        $(&quot;#id_building_error_parent_div&quot;).show();
                    }else{
                        $(&quot;#id_building_error_parent_div&quot;).hide();
                    }
                }
            }
        });

        $('#id_map_location').change(function(){
            // console.log($(this).val())
            if ($(this).val() == &quot;Others&quot;){
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    $(&quot;#id_map_location_building_div&quot;).show();
                }
                $(&quot;#id_map_add_extra_location&quot;).show();
            }else{
                $(&quot;#id_map_add_extra_location&quot;).hide();
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                        $(&quot;#id_map_location_building_div&quot;).show();
                    }else{
                        $(&quot;#id_map_location_building_div&quot;).hide();
                    }
                }
            }
        });

        $('#id_location_algo').change(function() {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_location_algo_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_location_algo_error_parent_div&quot;).hide();
            }
        });

        $('#id_cus_location').change(function() {
            if (this.value == &quot;Others&quot;){
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    $(&quot;#id_cus_building_error_parent_div&quot;).show();
                }
                $(&quot;#id_cus_extra_location_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_cus_extra_location_error_parent_div&quot;).hide();
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                        $(&quot;#id_cus_building_error_parent_div&quot;).show();
                    }else{
                        $(&quot;#id_cus_building_error_parent_div&quot;).hide();
                    }
                }
            }
        });

        $('#id_purpose_description').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).hide();
            }
        });

        $('#id_purpose_description_algo').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_purpose_description_algo_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_purpose_description_algo_error_parent_div&quot;).hide();
            }
        });

        $('#id_purpose_description_custom').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).hide();
            }
        });

        $('#id_cost_source').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_cost_source_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_cost_source_error_parent_div&quot;).hide();
            }
        });

        $('#id_cus_unit').on('change', function (e) {
            if (this.value == &quot;Othrs&quot;){
                $(&quot;#id_extra_cus_unit_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_cus_unit_error_parent_div&quot;).hide();
            }
        });

        $('#id_cus_unit_assembly').on('change', function (e) {
            if (this.value == &quot;Othrs&quot;){
                $(&quot;#id_extra_cus_unit_assembly_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_cus_unit_assembly_error_parent_div&quot;).hide();
            }
        });

        $('#id_recovery_period').change(function() {
            if (this.value == &quot;GDS_sc&quot;){
                $('#id_location').prop('disabled', true);
                $('#id_location').val('None')

            }else{
                $('#id_location').prop('disabled', false);
            }
        });

        $('#id_cus_recovery_period').change(function() {
            if (this.value == &quot;GDS_sc&quot;){
                $('#id_cus_location').prop('disabled', true);
                $('#id_cus_location').val('None')

            }else{
                $('#id_cus_location').prop('disabled', false);
            }
        });

        $(window).scroll(function () {
            if ($(this).scrollTop() > 50) {
                $('#back-to-top').fadeIn();
            } else {
                $('#back-to-top').fadeOut();
            }
        });

        $('#back-to-top').click(function () {
            $('#back-to-top').tooltip('hide');
            $('body,html').animate({
                scrollTop: 0
            }, 800);
            return false;
        });

        $('#back-to-top').tooltip('show');

        $('#changepassword').on('shown.bs.modal', function() {
            $('.cs-error').hide();
            $('.form-group').removeClass('has-error');
        });
    });

    $(&quot;#other_information_btn&quot;).click(function(){
        var selectedEffect = &quot;fade&quot;;
        $(&quot;.other_information&quot;).toggle( selectedEffect, 500, callback );
        $(this).html($(this).html() == 'Show Others Fields' ? 'Hide Others Fields' : 'Show Others Fields');
    });

    function callback() {
        setTimeout(function() {
            $( &quot;#effect&quot; ).removeAttr( &quot;style&quot; ).hide().fadeIn();
        }, 400 );
    };

    $(document).bind('keypress', function(e) {
        var code = e.shiftKey &amp;&amp; e.which;
        if (code === 69){
            if (!is_modal_open){
                ''
                    show_hide_child();
                ''
            }
        }
    });

    $.ajaxTransport(&quot;+binary&quot;, function(options, originalOptions, jqXHR){
        // check for conditions and support for blob / arraybuffer response type
        if (window.FormData &amp;&amp; ((options.dataType &amp;&amp; (options.dataType == 'binary'))
            || (options.data
            &amp;&amp; ((window.ArrayBuffer &amp;&amp; options.data instanceof ArrayBuffer)
            || (window.Blob &amp;&amp; options.data instanceof Blob))))
        )
        {
            return {
                // create new XMLHttpRequest
                send: function(headers, callback){
            // setup all variables
                    var xhr = new XMLHttpRequest(),
            url = options.url,
            type = options.type,
            async = options.async || true,
            // blob or arraybuffer. Default is blob
            dataType = options.responseType || &quot;blob&quot;,
            data = options.data || null,
            username = options.username || null,
            password = options.password || null;
                    xhr.addEventListener('load', function(){
                var data = {};
                data[options.dataType] = xhr.response;
                // make callback and send data
                callback(xhr.status
                        , xhr.statusText
                        , data
                        , xhr.getAllResponseHeaders());
                    });

                    xhr.open(type, url, async, username, password);

            // setup custom headers
            for (var i in headers ) {
                xhr.setRequestHeader(i, headers[i] );
            }

                    xhr.responseType = dataType;
                    xhr.send(data);
                },
                abort: function(){
                    jqXHR.abort();
                }
            };
        }
    });

    function revert_items_cost(){
        revert_ajax_request(&quot;/project/revert-cost-factors/&quot;, function(response){
            location.reload();
        });
    }

    function revert_items_cost_and_reopen(){
        revert_ajax_request(&quot;/project/revert-cost-factors/&quot;, function(response){
            window.location.href = window.location.href.split(&quot;?&quot;)[0] + '?page=input_order';
        })
    }

    function bulk_ajax_request($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;GET&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
            },
            success: $f
        });
    }

    function bulk_ajax_delete_request($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;POST&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
                'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;
            },
            success: $f
        });
    }

    function bulk_ajax_delete_request_filter($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;POST&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
                'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;,
                &quot;delete_filtered_zeroes&quot;: true
            },
            success: $f
        });
    }

    function revert_ajax_request($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;POST&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
                'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;
            },
            success: $f
        });
    }

    function update_totalcost_from_db_and_reopen(){
        bulk_ajax_request(&quot;/project/update-totalcost-db/&quot;, function(response){
            window.location.href = window.location.href.split(&quot;?&quot;)[0] + '?page=input_order';
        });
    }

    function duplicate_items_and_reopen(){
        bulk_ajax_request(&quot;/project/duplicate-items/&quot;, function(response){
            window.location.href = window.location.href.split(&quot;?&quot;)[0] + '?page=input_order';
        });
    }

    function downloadFile(url, headers, filename) {
        $(&quot;#id_processing&quot;).show();
        return $.ajax({
            url:url,
            method: &quot;POST&quot;,
            // dataType:&quot;binary&quot;,
            data: {
                'current': window.location.href,
                'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;
            },
            // processData: false,
            // headers:headers
            success: function(){
                // $(&quot;#id_processing&quot;).hide();
                $(&quot;#show-report-success&quot;).show();
                setTimeout(function(){$(&quot;#show-report-success&quot;).hide()}, 2000)
            }
        })
    }

    function exportsheet(property_pk) {
        // $(&quot;#id_processing&quot;).show();
        if(UpdateQuantityRequestInProgress.length &lt;= 0){
            var url = &quot;/project/export-excel-sheet/&quot;+ property_pk + '/';
            var sheet_name = 'April 21'
            sheet_name = sheet_name.replace(/[-!#$%^&amp;*()_+|~=`{}\[\]:&quot;;'&lt;>?,.\/]/g, '')
                                    .replace(/\s/g, '_')
            var filename = &quot;TakeoffSheet&quot; + sheet_name + &quot;.zip&quot;

            downloadFile(url, {&quot;X-Requested-With&quot;:&quot;XMLHttpRequest&quot;}, filename);
        }
    }

    
    var selectedIndex = [];
    

    function initializeSelectedIndex() {
        if (selectedIndex){
            for(index of selectedIndex){
                $(&quot;[select-data-id='&quot;+index+&quot;']&quot;).addClass(&quot;selected&quot;);
                $(&quot;.pre-dis&quot;).prop(&quot;disabled&quot;, false);
                // $(&quot;#bulk_tool_select&quot;).modal(&quot;show&quot;);
            }
            $(&quot;input[name='id_lists']&quot;).val(selectedIndex);
        }
    };

    $(&quot;#id_delete_item&quot;).on('click', function(e){
        var records = selectedIndex.length;
        $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
        $('#deleteConfirmModal').modal(&quot;show&quot;);
    });

    $(&quot;#update_totalcost_db&quot;).on('shown.bs.modal', function(e){
        var records = selectedIndex.length;

        $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
    });

    $('#warnings_dialogue_link').on('click', function(){
        var property_type = 'PPA'
        if(property_type == &quot;PPA&quot;){
            $('#warningdialogueNEWtoPPA').modal(&quot;show&quot;);
        } else if(property_type == &quot;NEW&quot;) {
            $('#warningdialoguePPAtoNEW').modal(&quot;show&quot;);
        }
    })

    $('#id_edit_bulk_con').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            $(&quot;#id_edit_bulk_extra_purpose_description&quot;).show();
        }else{
            $(&quot;#id_edit_bulk_extra_purpose_description&quot;).hide();
        }
    });

    $('#id_edit_bulk_filter_purpose').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            $(&quot;#id_filter_bulk_extra_purpose_description&quot;).show();
        }else{
            $(&quot;#id_filter_bulk_extra_purpose_description&quot;).hide();
        }
    });

    $('#id_edit_bulk_loc').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                $(&quot;#id_edit_bulk_building&quot;).show();
            }
            $(&quot;#id_edit_bulk_extra_location&quot;).show();
        }else{
            $(&quot;#id_edit_bulk_extra_location&quot;).hide();
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                    $(&quot;#id_edit_bulk_building&quot;).show();
                }else{
                    $(&quot;#id_edit_bulk_building&quot;).hide();
                }
            }
        }
    });

    $('#id_edit_bulk_loc_filter').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                $(&quot;#id_edit_bulk_building_filter&quot;).show();
            }
            $(&quot;#id_edit_bulk_extra_location_filter&quot;).show();
        }else{
            $(&quot;#id_edit_bulk_extra_location_filter&quot;).hide();
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                    $(&quot;#id_edit_bulk_building_filter&quot;).show();
                }else{
                    $(&quot;#id_edit_bulk_building_filter&quot;).hide();
                }
            }
        }
    });

    $(&quot;#id_update_qantity&quot;).on('click', function(e){
        $.ajax({
            url: &quot;/project/update-all-quantiity/&quot;,
            type: &quot;GET&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
            },
            success: function(response){
                location.reload();
            }
        });
    });

    $(document).on('dblclick', '[data-id]:not([assmb-item-id])', function(e){
        e.preventDefault();
        var self = $(this),
        item_id = self.attr('data-id');
        // $('#item_details_modal').remove();
        if (!read_only_view_enabled){
            $.ajax({
                url: &quot;/project/record-details/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'item_id' : item_id,
                    'property_id': property_id,
                    'view_mode': $(&quot;[name='view_mode']:checked&quot;).val()
                },
                success: function(response){
                    $(&quot;#item_details_modal&quot;).html(response.data);
                    is_modal_open = true
                    bindModalEvent();
                    $('#item_details').modal('show');
                    $('#id_edit_con').on('change', function (e) {
                        if (this.value == &quot;Others&quot;){
                            $(&quot;#id_edit_extra_purpose_description_error_parent_div&quot;).show();
                        }else{
                            $(&quot;#id_edit_extra_purpose_description_error_parent_div&quot;).hide();
                        }
                    });
                    $('#id_edit_loc').on('change', function (e) {
                        if (this.value == &quot;Others&quot;){
                            $(&quot;#id_edit_extra_location_error_parent_div&quot;).show();
                        }else{
                            $(&quot;#id_edit_extra_location_error_parent_div&quot;).hide();
                        }
                    });
                }
            });
        }else{
            e.stopPropagation();
        }
    });

    $(document).on('click', '[select-data-id]', function(e){

        e.preventDefault();
        var self = $(this),
        item_id = self.attr('select-data-id');
        // if(!read_only_view_enabled){
        if(e.shiftKey || e.ctrlKey) {
            e.stopPropagation();
            if(self.hasClass('selected')) {
                self.removeClass('selected');
                selectedIndex.splice(selectedIndex.indexOf(item_id), 1);
                $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);

            } else {
                self.addClass('selected');
                selectedIndex.push(item_id);
                $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);

            }
            if(selectedIndex.length === 0) {
                $(&quot;.pre-dis&quot;).attr(&quot;disabled&quot;, &quot;true&quot;);
            } else {
                $(&quot;.pre-dis&quot;).removeAttr(&quot;disabled&quot;);
            }
        }
        // }else{
        //     e.stopPropagation();
        // }

    });

    $(document).on('click', '[record-id]', function(e){
        if(read_only_view_enabled){
            e.preventDefault();
            e.stopPropagation();
        }else{
            $(this).prop('contenteditable', true);
        }

        //$('td[contenteditable]').focus(function(e) {
        //    document.execCommand('insertHTML', false);
        //    document.execCommand('selectAll', false);
        //    return false;
        //});

        //$('td[contenteditable]').keypress(function(e){ return e.which != 13; });
    });

    $(document).on('focus', '[record-id]', function(e){
        // document.execCommand('selectAll', false);
        $(this).selectText();
    });

    $(document).on('keypress', '[record-id]', function(e){ return e.which != 13; });

    $(document).on('dblclick', '[assmb-item-id]', function(e){
        var self = $(this),
        item_id = self.attr('assmb-item-id');
        e.preventDefault();

        if(!read_only_view_enabled){
            $.ajax({
                url: &quot;/project/assemb-item-details/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'item_id' : item_id,
                    'view_mode': $(&quot;[name='view_mode']:checked&quot;).val()
                },
                success: function(response){
                    $(&quot;#item_details_modal&quot;).html(response.data);
                    is_modal_open = true;
                    bindModalEvent();
                    $('#item_details').modal('show');
                }
            });
        }else{
            e.stopPropagation();
        }
        // $('#item_details_modal').remove();
    });

    var selectAllItems = true;
    $('#id_select_item').on('click',async function(e) {
        if(is_all_items_loaded){
            e.preventDefault();
            select_all_items();
            $(&quot;#id_processing&quot;).hide();
        }
        else{
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#id_processing&quot;).show();
            res = await new Promise(function(resolve, reject) {
                setInterval(function(){ if(is_all_items_loaded){resolve(&quot;Done!&quot;)}; }, 3000);
            });
            $(this).click();
        }
    });

    $('#id_select_inverse_item').on('click', async function(e){
        if(is_all_items_loaded){
            e.preventDefault();
            select_all_inverse_items();
            $(&quot;#id_processing&quot;).hide();
        }else{
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#id_processing&quot;).show();
            res = await new Promise(function(resolve, reject) {
                setInterval(function(){ if(is_all_items_loaded){resolve(&quot;Done!&quot;)}; }, 3000);
            });
            $(this).click();
        }
    });

    // **************************************** search for main items******************************************//

    $(function(){
        typeaheadRecords({
            element: '#id_item',
            url: '/project/search-item/',
            extra: '#id_extra_item'
        })
        window.UpdateTableHeader = tableFixedHeader('.fixed-header');

        $(window).on('resize', function() {
            if (typeof UpdateTableHeader === 'function') {
                UpdateTableHeader.call();
            }
        })
    });

    // **************************************** search for custom items******************************************//

    $(function(){
        typeaheadRecords2({
            element: '#id_custom_item_search',
            url: '/project/search-custom-item/'
        })
    });

    // ************************************** search for custom asseblies items *********************************//

    $(function(){
        typeaheadRecords3({
            element: '#id_search_items',
            url: '/project/search-item/',
            extra: '#id_extra_item_1'
        })
    });

    // ***************************************** end of search functionality ************************************//

    function executeAsync() {
      return new Promise(resolve => {
         intrvl = setInterval(function(){ if(is_all_items_loaded){
            initializeSelectedIndex();
            clearInterval(intrvl)
            resolve(&quot;Done!&quot;)
        }; }, 3000);
      });
    }

    async function asyncCall() {
      const result = await executeAsync();
      // expected output: 'resolved'
    }

    $(document).ready(function(){
        
            executeAsync();
        
        moveNextQuantityEventBind();

        if(toggleCost == 'true'){
            toggle_cost();
        }
        $('#id_additem_form').submit(function(e){
            save_input($(this));
        });
        $('#id_cus_additem_form').submit(function(e){
            save_input($(this));
        });
    });

    $(document).on('subconstruction_loaded', function(){
        inputs = localStorage.getItem('inputs');
        if (inputs){
            inputs = JSON.parse(inputs);
        }
        if(inputs &amp;&amp; inputs['id_item_sub_group_no_items'] &amp;&amp; inputs['id_item_sub_group_no_items'] != 'null')
        {
            document.getElementById(&quot;id_item_sub_group_no_items&quot;).value = inputs['id_item_sub_group_no_items']
            $('#id_item_sub_group_no_items').trigger('change');
            $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;)
            $(&quot;.item--suggestion-2&quot;).css(&quot;display&quot;, &quot;none&quot;)
        }
        else{
            $(&quot;#id_item_sub_group_no_items&quot;).val('');
            subgroup = ''
        }
    })
    $(document).on('subconstruction_loaded1', function(){
        inputs = localStorage.getItem('inputs');
        if (inputs){
            inputs = JSON.parse(inputs);
        }
        if(inputs &amp;&amp; inputs['id_item_sub_group_no_items_1'])
        {
            document.getElementById(&quot;id_item_sub_group_no_items_1&quot;).value = inputs['id_item_sub_group_no_items_1']
            $('#id_item_sub_group_no_items_1').trigger('change');
            $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;)
            $(&quot;.item--suggestion-2&quot;).css(&quot;display&quot;, &quot;none&quot;)
        }
        else{
            $(&quot;#id_item_sub_group_no_1&quot;).val('');
            subgroup = ''
        }
    })
    $(document).on('item_subgroup', function(){
        inputs = localStorage.getItem('inputs');
        if (inputs){
            inputs = JSON.parse(inputs);
        }
        if(inputs &amp;&amp; inputs['id_item_sub_group_no'] &amp;&amp; inputs['id_item_sub_group_no'] != 'null')
        {
            $(&quot;#id_item_sub_group_no&quot;).val(inputs['id_item_sub_group_no'])
            $('#id_item_sub_group_no').trigger('change');
            $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;)
            $(&quot;.item--suggestion-2&quot;).css(&quot;display&quot;, &quot;none&quot;)
            subgroup = $(&quot;#id_item_sub_group_no&quot;).val()
        }
        else{
            $(&quot;#id_item_sub_group_no&quot;).val('');
            subgroup = ''
        }
    })

    $('#id_additem_form :input').change(function(e){
        $('#id_additem_form button').removeAttr('disabled');
    });

    $(&quot;.bulk-tools, .pre-dis&quot;).on(&quot;click&quot;, async function(e){
        if(is_all_items_loaded){
            $(&quot;#id_processing&quot;).hide();
        }
        else{
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#id_processing&quot;).show();
            res = await new Promise(function(resolve, reject) {
                setInterval(function(){ if(is_all_items_loaded){resolve(&quot;Done!&quot;)}; }, 3000);
            });
            $(this).click();
        }

    })

    $(&quot;#notificationConfirmModal&quot;).on(&quot;hide.bs.modal&quot;, function(){
        window.location.reload();
    })

    var bulk_tool_temp;

    RowModel = (function(){

        function RowModel(model_json){
            for(key in model_json) {
                this[key] = model_json[key];
            }
        }

        return RowModel;
    })();

    var RowViewModel = (function () {
        function RowViewModel(form, model, formViewModel, rowId, status, validation_rule, change_rule) {
            this.form = form;
            this.model = model;
            this.rowId = rowId;
            this.validation_rule = validation_rule;
            this.change_rule = change_rule;
            this.status = status;

            /*
                {
                    'name': function(value){

                    },
                    ''
                }

                {
                    'name': function(rowId){
                        return {
                            is_valid: is_valid,
                            error_message: error_message
                        }
                    }
                }
            */

            this.formViewModel = formViewModel;

            this.view = new RowView(form, this, this.formViewModel.rowTemplate, status, rowId);
            if (status == 'updated'){
                my_event = $.Event( &quot;change&quot;, { data: { newly_created: true } } )
            }
            else{
                my_event = $.Event( &quot;change&quot;)
            }

            $ths = this;
            $(this.view.rowElement).find(':input').each(function(){$ths.ChangedInView($(this).attr('name'), my_event)});
        }

        RowViewModel.prototype.ChangedInView = function (field_name, e) {
            old_value = this.model[field_name];
            $('.asset_class_setup').multiselect({
                buttonWidth: '100%',
                nonSelectedText: 'Select',
                enableFiltering: true,
                enableCaseInsensitiveFiltering: true,
                maxHeight: 300,
            });

            $(&quot;.boot-multiselect-demo&quot;).multiselect(
                {
                    nonSelectedText: 'Select',
                    numberDisplayed: 1,
                    nSelectedText: &quot;selected&quot;,
                    buttonWidth: &quot;100%&quot;,
                }
            );

            if (e.data == undefined){
                if (field_name == &quot;asset_class&quot;){
                    var value = $('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').val();
                    var $this = $('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]');
                    var row_id = this.rowId
                    var $this = this

                    // if (value){
                        // $.ajax({
                        //     type: 'GET',
                        //     url: '/project/map-asset-class-cost-setup/',
                        //     data:{
                        //         'asset_id': value,
                        //         'property_id': '3338',
                        //     },
                        //     success: function(response){
                        //         if (response.recovery_code){
                        //             $('#row-'+row_id+ ' [name=&quot;recovery_period&quot;]').val(response.recovery_code)

                        //             var new_field_name = &quot;recovery_period&quot;
                        //             $this.model[new_field_name] = response.recovery_code
                        //             if($this.change_rule.hasOwnProperty(new_field_name)){
                        //                 new_value = $this.model[new_field_name];
                        //                 $this.change_rule[new_field_name]('#row-'+row_id, old_value, new_value, $this, e);
                        //             }
                        //         }
                        //     }

                        // });
                    // }
                }
            }
            if($('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').is('input[type=&quot;checkbox&quot;]')){
                if($('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').prop('checked')){
                    value = 'on'
                }
                else{
                    value = 'off'
                }
                this.model[field_name] = value;
            }else{
                this.model[field_name] = $('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').val();
            }

            if(this.change_rule.hasOwnProperty(field_name)){
                new_value = this.model[field_name];
                this.change_rule[field_name]('#row-'+this.rowId, old_value, new_value, this, e);
            }
        };

        RowViewModel.prototype.ChangedInViewModel = function (field_name, field_value, newly_created=false) {

            this.model[field_name] = field_value;

            this.view.ChangedInViewModel(field_name, field_value, newly_created);
        };

        RowViewModel.prototype.validate = function() {
            valid = true;
            row_validation_result = {}
            for(field_name in this.validation_rule){
                validation_result = this.validation_rule[field_name](this.model[field_name], this.model);
                valid = valid &amp;&amp; validation_result[0]
                row_validation_result[field_name] = validation_result;
            }
            this.show_validation_result(row_validation_result);
            return valid;
        };

        RowViewModel.prototype.show_validation_result = function(row_validation_result) {
            var _this = this;
            Object.keys(row_validation_result).forEach(function(value){
                if(!row_validation_result[value][0]){
                    $('#row-'+_this.rowId+&quot; [name='&quot;+value+&quot;']&quot;).parent(&quot;div&quot;).addClass('has-error');
                }
                else{
                    $('#row-'+_this.rowId+&quot; [name='&quot;+value+&quot;']&quot;).parent(&quot;div&quot;).removeClass('has-error');
                }
            })
        }

        RowViewModel.prototype.deleteRow = function () {
            this.formViewModel.deleteRow(this.rowId);
        }

        return RowViewModel;
    })();

    test = {}
    var RowView = (function () {
        function RowView(form, viewModel, template, status, rowId) {
            this.form = form;
            $(form).append(&quot;&lt;div id='row-&quot; +rowId +&quot;' class='row &quot;+ status +&quot;'>&lt;/div>&quot;)
            this.rowElement = document.getElementById('row-'+rowId)
            this.viewModel = viewModel;
            var _this = this;

            for (element_index in template) {
                $(this.rowElement).append(template[element_index].clone());
            }

            for (field_name in this.viewModel.model) {
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').val(this.viewModel.model[field_name]);
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').on(&quot;change keyup keypress keydown&quot;, this.ChangedInView(field_name));
            }

            $(this.rowElement).find('.delete').on(&quot;click&quot;, function () {
                return _this.deleteRow();
            });
        }

        RowView.prototype.ChangedInView = function (field_name) {
            var _this = this;
            return function (e){
                _this.viewModel.ChangedInView(field_name, e);
            }
        };

        RowView.prototype.ChangedInViewModel = function (field_name, field_value, newly_created=false) {
            if($(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').is('input[type=&quot;checkbox&quot;]')){
                if(field_value=='on'){
                    $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').prop('checked', 'checked')
                }
                else{
                    $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').prop('checked', false);
                }
            }
            if(field_name == 'asset_class'){
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').multiselect('select', field_value)
            }

            if(field_name == 'indirect_cost_sources'){
                console.log(field_value, 'check')
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').multiselect('select', field_value)
            }

            $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').val(field_value);

            if(field_name != 'subgroup'){
                // console.log($(this.rowElement).find('[name=&quot;'+field_name+'&quot;]'))
                // console.log($(this.rowElement))
                test[field_name] = this.rowElement
                // console.log(field_name)
                my_event = $.Event( &quot;change&quot;, { data: { newly_created: newly_created } } )
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').trigger(my_event);
            }else{
                // console.log($(this.rowElement).find('[name=&quot;'+field_name+'&quot;]'))
                // console.log(field_name)
            }
        };

        RowView.prototype.deleteRow = function () {
            var value = $(this.rowElement).children('[data-name=&quot;total_cost&quot;]').children('input[name=&quot;total_cost&quot;]').val().replace(/,/g, '');
            var total_value = $('#sum_total_cost_setup').val().replace(/,/g, '');
            var new_value = parseFloat(total_value) - parseFloat(value)
            var setup_value = new_value + parseFloat($('#cost_setup_sum').val())
            $('#sum_total_cost_setup').val(new_value.toFixed(2).toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;))
            $('#sum_contract_total').val(setup_value.toFixed(2).toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;))
            $(this.rowElement).remove();
            this.viewModel.deleteRow();
        };

        return RowView;
    })();

    var FormView = (function (){
        function FormView(form, viewModel, addButton, saveButton) {
            this.form = form;
            this.viewModel = viewModel;
            var _this = this;

            $(addButton).on(&quot;click&quot;, function () {
                return _this.addRow();
            });

            $(saveButton).on(&quot;click&quot;, function () {
                return _this.save($(this).data('url'));
            });
        }

        FormView.prototype.ChangedInView = function (field_name) {
            return function (field_value){_this.viewModel.ChangedInView(field_name, field_value);}
        };

        FormView.prototype.ChangedInViewModel = function (field_name, field_value) {

            $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').val(field_value);
        };

        FormView.prototype.addRow = function () {
            this.viewModel.addRow();
        };

        FormView.prototype.save = function (next_url) {
            this.viewModel.save(next_url);
        };

        return FormView;
    })();

    var FormViewModel = (function () {
        function FormViewModel(form, addButton, saveButton, validation_rule, change_rule, previous_object, rowTemplate, modelTemplate, globalStatesTemplate, stateFunction, rowFunction, success_url, sortorder) {
            this.form = form;
            this.model = {};
            this.rowTemplate = rowTemplate;
            this.view = new FormView(form, this, addButton, saveButton);
            this.validation_rule = validation_rule;
            this.change_rule = change_rule;
            this.modelTemplate = modelTemplate;
            this.globalStatesTemplate = globalStatesTemplate;
            this.stateFunction = stateFunction;
            this.rowFunction = rowFunction;
            this.success_url = success_url;
            this.saveButton = saveButton;

            this.globalStates = {}

            for(state in this.globalStatesTemplate){
                this.globalStates[state] = {}
                for(id in this.globalStatesTemplate[state]){
                    this.globalStates[state][id] = {
                        'status': 'existing',
                        'value': this.globalStatesTemplate[state][id]
                    }
                }
            }

            for(state_manager in this.stateFunction){
                this.stateFunction[state_manager].initialize(this);
            }

            console.log(previous_object, 'previous')

            for (object in previous_object){
                this.addRow(object, previous_object[object]);
            }

            $.each(sortorder,function(index,value){
                if($.inArray(value, sortorder)!==-1){
                    $(form).append($('#'+value));
                }
            });

            $(form+' div').each(function(){
                if($.inArray($(this).attr('id'), sortorder)==-1){
                    // inArray will return -1, if the element was not found in array
                    $(form).append($('#'+$(this).attr('id')));
                }
            });
        }

        FormViewModel.prototype.addState = function(name, id, label, extra_context){
            var _this =this;
            if(!this.globalStates[name].hasOwnProperty(id)){
                this.globalStates[name][id] = {
                    'status': 'new',
                    'value': ''
                }
                this.globalStates[name][id]['value'] = label;
                this.stateFunction[name].added(_this, id, label, extra_context);
            }
            else{
                this.updateState(name, id, label, extra_context);
            }
        }

        FormViewModel.prototype.updateState = function(name, id, label, extra_context){
            var _this = this;
            this.globalStates[name][id]['value'] = label;
            this.stateFunction[name].updated(_this, id, label, extra_context);
        }

        FormViewModel.prototype.removeState = function(name, id, extra_context){
            var _this = this;
            delete this.globalStates[name][id];
            this.stateFunction[name].removed(_this, id, extra_context);
        }

        FormViewModel.prototype.addRow = function (rowId, data_model){
            $this = this

            var model_template_value = JSON.parse(JSON.stringify(this.modelTemplate))
            var model_list = Object.keys(this.model).filter(function(x){ return $this.model[x].status != 'deleted' })
            if (model_list.length >= 1 &amp;&amp; data_model == undefined){
                var last_row_id = model_list[model_list.length - 1]
                model_template_value['cus_unit'] = this.model[last_row_id]['model']['cus_unit']
                model_template_value['quantity'] = this.model[last_row_id]['model']['quantity']
                model_template_value['cost_source'] = this.model[last_row_id]['model']['cost_source']
                model_template_value['item_group_no'] = this.model[last_row_id]['model']['item_group_no']
                model_template_value['recovery_period'] = this.model[last_row_id]['model']['recovery_period']
                model_template_value['location'] = this.model[last_row_id]['model']['location']
                model_template_value['indirect_cost_sources'] = this.model[last_row_id]['model']['indirect_cost_sources']
                model_template_value['asset_class'] = this.model[last_row_id]['model']['asset_class']
            }

            var model = new RowModel(model_template_value)

            if(!rowId){
                uuid = uuidv4();
                status = 'created';
            }
            else{
                uuid = rowId;
                status = 'updated';
            }
            var new_row = new RowViewModel(this.form, model, this, uuid, status, this.validation_rule, this.change_rule);
            this.model[uuid] = new_row;

            if(data_model){
                for(field_name in data_model){
                    this.model[uuid].ChangedInViewModel(field_name, data_model[field_name], newly_created=true)
                }
            }

            if(status == 'updated'){
                this.rowFunction.rowAdded(this, rowId)
            }
        }

        FormViewModel.prototype.save = function(next_url) {
            is_valid = true;
            for(row in this.model){
                if(this.model[row].status != 'deleted'){
                    row_valid = this.model[row].validate();
                    is_valid = is_valid &amp;&amp; row_valid;
                }
            }
            var _this = this;

            if(is_valid){
                $(_this.saveButton).html(
                    '&lt;img src=&quot;/static/images/spin.svg&quot; style=&quot;height: 25px; margin-top: -5px;&quot;>')
                $(_this.saveButton).attr(&quot;disabled&quot;, true);

                body = {
                    'created': {},
                    'deleted': [],
                    'updated': {},
                    'global_states':{}
                }
                for(row in this.model){

                    if(this.model[row].status == 'created'){
                        body['created'][this.model[row].rowId] = this.model[row].model;
                    }
                    else if(this.model[row].status == 'deleted'){
                        body['deleted'].push(this.model[row].rowId);
                    }
                    else{
                        body['updated'][this.model[row].rowId] = this.model[row].model;
                    }
                }

                body['global_states'] = this.globalStates;
                body[&quot;order&quot;] = $(this.form).sortable(&quot;toArray&quot;);

                $.ajax({
                    url: window.location.href,
                    type: 'post',
                    success: function (data) {
                        window.location.href = next_url;
                    },
                    data: {
                        'body': JSON.stringify(body),
                        'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;
                    }
                });
            }
        }

        FormViewModel.prototype.deleteRow = function (rowId){
            this.rowFunction.rowDeleted(this, rowId)
            if(this.model[rowId].status == 'created'){
                delete this.model[rowId];
            }
            else if (this.model[rowId].status == 'updated'){
                this.model[rowId].status = 'deleted';
            }
        }

        return FormViewModel;
    })();

    jQuery.fn.selectText = function(){
       var doc = document;
       var element = this[0];
       if (doc.body.createTextRange) {
           var range = document.body.createTextRange();
           range.moveToElementText(element);
           range.select();
       } else if (window.getSelection) {
           var selection = window.getSelection();
           var range = document.createRange();
           range.selectNodeContents(element);
           selection.removeAllRanges();
           selection.addRange(range);
       }
    };

    $(document).on(&quot;click&quot;, &quot;.analysis_window_tab&quot;, function(){
        if(!$(this).hasClass(&quot;active&quot;)){
            property_id = $(this).data(&quot;property&quot;);
            tab = $(this).data(&quot;tab&quot;);
            estimate_result(property_id, tab=tab);
        }
    })

    var NotificationManager = (function () {
        function NotificationManager() {
            this.notification_container = &quot;.notification-container&quot;;
            this.is_container_open = false;
            this.limit = 4;
            this.offset = 0;
            this.has_more = true;
            this.loading = false;
            this.url_to_new_content = &quot;/get-notifications/&quot;;
            this.last_notification_id = null;
            this.new_notification = false;
            this.unread_count = null;
            var $this = this;
        }

        NotificationManager.prototype.get_notifications = function(popup=false){
            var $this = this;
            if($this.has_more){
                data = {
                    'last_notification_id': $this.last_notification_id,
                    'limit': $this.limit,
                    'offset': $this.offset,
                    'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;,
                    'popup': popup
                }
                $.ajax({
                    type:'POST',
                    url: $this.url_to_new_content,
                    data: data,
                    success:function(data){
                        $this.loading = false;
                        $this.has_more = data.has_more;
                        $this.offset = $this.offset + $this.limit;
                        $this.last_notification_id = data.last_notification_id;
                        $($this.notification_container).append(data.content);
                        $(this.notification_container).scroll()
                    }
                });
            }
        }

        NotificationManager.prototype.update_notification = function(data){
            // $(&quot;#id-num&quot;).html(data.count)
            $(&quot;.id-num&quot;).html(data.count)
            if(this.is_container_open){
                this.show_refresh_option();
            }else{
                $(&quot;#id-comment&quot;).show();
                $(&quot;.id-num&quot;).show();
            }
        }

        NotificationManager.prototype.reset_notification = function(){
            this.last_notification_id = null;
            this.offset = 0;
            this.has_more = true;
            $(this.notification_container).html(&quot;&quot;);
        }

        return NotificationManager;
    })();

    var notification_manager;

     $(document).ready(function(){
        notification_manager = new NotificationManager();
        
        setupWebSocket()
        

        $('.import_cost_setup').on('click', function(e){
            var id = $(this).data('id');
            var type = $(this).data('type');

            $('#upload_cost_setup').attr('onclick', 'uploadCostFile(&quot;3338&quot;, &quot;'+type+'&quot;)');

            $('#importCostSetup').modal('show');


        });
    })

    var wsUri = &quot;wss://dev.segstream.com/ws/?user_id=32&quot;;
    function setupWebSocket() {
        websocket = new WebSocket(wsUri) ;
        websocket.onopen = function(evt) { onopen(evt) } ;
        websocket.onmessage = function(evt) { onMessage (evt)};
        websocket.onclose = function(e) {
            console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
            setTimeout(function() {
              setupWebSocket();
            }, 1000);
          };
    };

    function onopen (evt) {

        // websocket.send('{&quot;type&quot;: &quot;ping&quot;}')
    }

    function popupNotificationName(name, next_url){
        $('#notification_property_name').html(name);
        $('#next_link_worksheet').attr('href', next_url);
        $('#notificationConfirmModal').modal('show');

        setTimeout(function(){
            localStorage.setItem(&quot;notification_modal&quot;, false);
        }, 5000);
    }

    function onMessage (evt) {
        data = JSON.parse(evt.data);
        if(data.type == 'notification'){

            
            var notification_property_id = 3338;
            

            if(!data.property_id || (data.property_id != notification_property_id)){
                if(data.notification_type != 'export'){
                    localStorage.setItem(&quot;notification_modal&quot;, true);

                    localStorage.setItem(&quot;property_name&quot;, data.property_name);
                    localStorage.setItem(&quot;next_url&quot;, data.next_url);
                    // profile_manager.open_up_notification_popup(data);
                    popupNotificationName(data.property_name, data.next_url)
                }
            }


        }
        if(data.type == 'export'){
            if (window.location.href == data.location){
                // window.location.href = data.data;

                var url = data.data;
                var sheet_name = 'April 21'

                var filename = data.filename;

                // var request = new XMLHttpRequest();
                // request.open('GET', url, true);
                // request.responseType = 'blob';
                // request.onload = function() {
                //    var link = document.createElement('a');
                //    document.body.appendChild(link);
                //    link.href = window.URL.createObjectURL(request.response);
                //    link.download = filename;
                //    link.click();
                // };
                // request.send();

                var link = document.createElement('a');
               document.body.appendChild(link);
               link.href = url;
               link.download = filename;
               link.click();
                $(&quot;#id_processing&quot;).hide();

                $.ajax({
                    'type': &quot;GET&quot;,
                    &quot;url&quot;: &quot;/read/report/&quot;+data.report_id
                })
            }
            else{
                    profile_manager.open_up_notification_popup({'data': {
                    'count': &quot;!&quot;
                }});
            }
        }

        if(data.type == 'Algorithm Verify'){
            $(&quot;#verify_result&quot;).html(data.message);
            $(&quot;#verify_result&quot;).show();
            $(&quot;.statement&quot;).each(function(){
                n_width = $(this).children(&quot;.statement_name&quot;).css(&quot;width&quot;);
                v_width = $(this).children(&quot;.statement_value&quot;).css(&quot;width&quot;);
                n_width = parseFloat(n_width.slice(0, n_width.length-2))
                v_width = parseFloat(v_width.slice(0, v_width.length-2))
                if(n_width > v_width){
                    $(this).children(&quot;.statement_name, .statement_value&quot;).css(&quot;width&quot;, n_width)
                }else{
                    $(this).children(&quot;.statement_name, .statement_value&quot;).css(&quot;width&quot;, v_width)
                }
                $(this).children(&quot;.statement_name&quot;).css(&quot;display&quot;, &quot;inline-block&quot;)
                $(this).children(&quot;.statement_value&quot;).css(&quot;display&quot;, &quot;none&quot;)
            })
        }

        if(data.type == &quot;Algorithm Component Extracted&quot;){
            $(&quot;#component-ready-&quot;+data.id).text(data.count)
        }

        if(data.type == &quot;Enable Refresh Algorithm&quot;){
            $('#refresh_algorithm').prop('disabled', false);
        }

        if(data.type == &quot;Algorithm Item Completed&quot;){

            $.ajax({
                type: 'POST',
                url: '/algorithm-component/',
                data: {
                    'item_id': data.id,
                    'property_id': data.property_id,
                    'csrfmiddlewaretoken': &quot;h7rxFMhtwfTobL1Jzt9hIjDb2AUjUBe6qpvQtfi8JHam6Y8WZtzmv9x3jZAgHSw5&quot;
                },
                success: function(response){
                    $(&quot;#component-not-ready-row-&quot;+response.id).replaceWith(response.components)
                }
            });
        }

        if(data.type == &quot;enhanced_property_algorithm_completed&quot;){
            $(&quot;#&quot;+data.algorithm_id+&quot;-processing-icon&quot;).hide()
            $(&quot;#&quot;+data.algorithm_id+&quot;-saved-icon&quot;).show()

        }

        if(data.type == &quot;enhanced_property_algorithm_entered&quot;){
            $(&quot;#&quot;+data.algorithm_id+&quot;-processing-icon&quot;).show()
        }

        if(data.type == &quot;building_processed&quot;){
            if(data.all_building_submit){
                building_ui.move_to_sheet()
            }
            building_ui.saved(data.building_id);
        }
    }

    var AlgorithmIntervalMap = {}
    var animateHeader = (function() {
        function constructor(algorithm_id){
            this.algorithm_id = algorithm_id;

            this.interval = setInterval(function(){$(&quot;.algorithm-list #algorithm-&quot;+algorithm_id).animate({
                opacity: $(&quot;.algorithm-list #algorithm-&quot;+algorithm_id).css('opacity') == 0.5 ? 0.8 : 0.5
            }, 250);}, 250)
        }
        return constructor;
    })();

    var user_clicked_notification_popup = false;
    function checkInteractionWithNotificationPopup() {
      return new Promise(resolve => {
        count = 0;
        $(&quot;.myBar, .myProgress&quot;).show();
        xinterval = setInterval(function(){
            var progressBarWidth = (10000-count) * $(&quot;.myProgress&quot;).width() / 10000;
            $(&quot;.myBar&quot;).css({&quot;width&quot;: progressBarWidth})
            if(count >= 10000){
                if(!user_clicked_notification_popup){

                close_notification_popup_and_show_count();
                $(&quot;#dropdown_menu_container&quot;).removeClass(&quot;notification_popup&quot;)
        $(&quot;.myBar, .myProgress&quot;).hide();
                profile_manager.status = &quot;closed&quot;;
            }
                clearInterval(xinterval);
                user_clicked_notification_popup = false
                resolve(&quot;Done!&quot;)
            }
            else{
                if(!isHovering(&quot;#dropdown_menu_container&quot;)){
                    count += 250
                }else{
                    count +=0;
                }
            }
        }, 250)
      });
    }

    async function hideNotif() {
      const result = await checkInteractionWithNotificationPopup();
      // expected output: 'resolved'
    }

    $(document).ready(function(){
        $('#notification-menu').on('shown', function () {
            notification_manager.is_container_open = true;
            notification_manager.reset_notification();
            notification_manager.get_notifications();
            $(&quot;#id-comment&quot;).hide();
            $(&quot;#id-num&quot;).hide();
        });

        $('#notification-menu').on('shown_notif', function () {
            notification_manager.is_container_open = true;
            notification_manager.reset_notification();
            notification_manager.get_notifications();
        });


        $('#notification-menu').on('hidden', function () {
            notification_manager.is_container_open = false;
        });

        notification_modal = localStorage.getItem(&quot;notification_modal&quot;);

        if (notification_modal == &quot;true&quot;){
            var name = localStorage.getItem(&quot;property_name&quot;);
            var next_url = localStorage.getItem(&quot;next_url&quot;);
            popupNotificationName(name, next_url);

        }
    });

    $('#navbarDropdown').on('click', function (event) {
        $(this).parent().toggleClass('open');
        if($(this).parent().hasClass(&quot;open&quot;)){
            $('#notification-menu').trigger(&quot;shown&quot;);
        }
        else{
            $('#notification-menu').trigger(&quot;hidden&quot;);
        }
    });

    var ProfileManager = (function () {
        function ProfileManager() {
            this.status = &quot;closed&quot; // Possible options: root, notification, management, notification_popup
            this.notification_count = null;
            this.home_div = &quot;#home-options&quot;;
            this.management_div = &quot;#management-options&quot;;
            this.dropdown_menu_container = &quot;#dropdown_menu_container&quot;
            this.notification_div = &quot;#notification-menu&quot;;
        }

        ProfileManager.prototype.open_management = function(){
            profile_manager.status = 'management';
            $(profile_manager.dropdown_menu_container).html($(profile_manager.management_div).html())
        }

        ProfileManager.prototype.open_notification = function(){
            profile_manager.status = 'notification';
            $(&quot;.id-num, .id-comment&quot;).hide();
            notification_manager.reset_notification();
            notification_manager.get_notifications();
            $(profile_manager.dropdown_menu_container).html($(profile_manager.notification_div).html())
        }

        ProfileManager.prototype.go_home = function(){
            profile_manager.status = 'root';
            $(profile_manager.dropdown_menu_container).html($(profile_manager.home_div).html())
        }

        ProfileManager.prototype.open_up_notification_popup = function(data){
            notification_manager.update_notification(data.data)
            this.status = &quot;notification_popup&quot;;
            $('#dropdown_link').click();
            newNotification = true;
            userInteractedWithNotificationPopup = false;
            hideNotif();
        }

        return ProfileManager;
    })();

    var profile_manager = new ProfileManager();

    $('#nav-drpdwn2').on('show.bs.dropdown', function(){
        if(profile_manager.status == &quot;notification_popup&quot;){
            $(profile_manager.dropdown_menu_container).addClass(&quot;notification_popup&quot;);
            notification_manager.reset_notification();
            notification_manager.get_notifications(popup=true);
            $(profile_manager.dropdown_menu_container).html($(profile_manager.notification_div).html())
        }
        else{
            profile_manager.status = 'root';
            $(profile_manager.dropdown_menu_container).html($(profile_manager.home_div).html())
        }
    });

    $(document).on(&quot;click&quot;, &quot;.management-options-link&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        profile_manager.open_management();
    })

    $(document).on(&quot;click&quot;, &quot;.notification-panel-link&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        profile_manager.open_notification();
    })

    $(document).on(&quot;click&quot;, &quot;.go-back-to-home&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        profile_manager.go_home();
    })

    $(document).on(&quot;click&quot;, &quot;.notification_popup&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        $(this).removeClass(&quot;notification_popup&quot;);
        $(&quot;.myBar, .myProgress&quot;).hide();
        profile_manager.open_notification();
        user_clicked_notification_popup = true;
    })

    $('#nav-drpdwn2').on(&quot;hide.bs.dropdown&quot;, function(e){
        if($(&quot;#dropdown_menu_container&quot;).hasClass(&quot;notification_popup&quot;)){
            e.preventDefault();
            e.stopPropagation();
        }
    })

    $(function(){

        $('*').hover(function(){
            $(this).data('hover',1); //store in that element that the mouse is over it
        },
        function(){
            $(this).data('hover',0); //store in that element that the mouse is no longer over it
        });


        window.isHovering = function (selector) {
            return $(selector).data('hover')?true:false; //check element for hover property
        }
    });

    $(window).on(&quot;unload&quot; ,function() {
        if(profile_manager.status == &quot;notification_popup&quot;){
            window.localStorage.setItem(&quot;notification_popup&quot;, &quot;true&quot;);
        }
    })

    $(document).ready(function(){
        if(localStorage.getItem(&quot;notification_popup&quot;) == &quot;true&quot;){
            
            var show_notif = false;
            

            if(show_notif){
                profile_manager.open_up_notification_popup({'data': {
                    'count': &quot;0&quot;
                }})
            }
            localStorage.setItem(&quot;notification_popup&quot;, &quot;false&quot;)
        }
    })

    function checkValue(value){
        if ($(value).val()){
            $(&quot;.search-form .form-group&quot;).addClass('hover');
        }else{
            $(&quot;.search-form .form-group&quot;).removeClass('hover');
            $('span.form-control-remove').removeClass('search-remove-active').addClass('search-remove-hide');
        }
    }

    function inputValue(){
        $(&quot;.search-form .form-group&quot;).addClass('hover');
    }

    function close_notification_popup_and_show_count(){
        $('#nav-drpdwn2').removeClass('open');
        $('#notification-menu').trigger(&quot;hidden&quot;)
        $(&quot;.id-num&quot;).show();
        $(&quot;id-comment&quot;).show();
    }

    $(document).on(&quot;click&quot;, &quot;.close-notification-popup&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        close_notification_popup_and_show_count();
    })

    var global_check = true

    function globalCheck(check){
        global_check = check
    }

    $(document).on(&quot;click&quot;, &quot;.prev_items_with_assemblies&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');

        if($(this).data('check_url') == 'custom'){
            $('#cus_button_clicked').val(check_next);
            update_cus_item_details($('#id_cus_additem_form'), async=false);
        }else{
            update_item_details($('#id_update_item_form'));
        }

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index &lt; 1){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        if(global_check){
            $(&quot;tr > td[data-id]&quot;).eq(index-1).dblclick();
        }
        $('#cus_button_clicked').val('');
    })

    $(document).on(&quot;click&quot;, &quot;.prev_items_with_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index &lt; 1){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr > td[data-id]&quot;).eq(index-1).dblclick();
        $('#button_clicked').val('');

    })


    $(document).on(&quot;click&quot;, &quot;.next_items_with_assemblies&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');

        if($(this).data('check_url') == 'custom'){
            $('#cus_button_clicked').val(check_next);
            update_cus_item_details($('#id_cus_additem_form'), async=false);
        }else{
            update_item_details($('#id_update_item_form'));
        }

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index >= $(&quot;tr > td[data-id]&quot;).length ){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        if(global_check){
            $(&quot;tr > td[data-id]&quot;).eq(index+1).dblclick();
        }
        $('#cus_button_clicked').val('');
    })

    $(document).on(&quot;click&quot;, &quot;.next_items_with_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index >= $(&quot;tr > td[data-id]&quot;).length ){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr > td[data-id]&quot;).eq(index+1).dblclick();
        $('#button_clicked').val('');
    })

    $(document).on(&quot;click&quot;, &quot;.prev_items_without_assemblies&quot;, function(e){
        e.preventDefault();
        if($(this).data('check_url') == 'assembly'){
            update_item_details($('#id_update_item_form'));
        }

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = (&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2-1).dblclick();
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2-1).dblclick();
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index-1).dblclick();
        return
    })

    $(document).on(&quot;click&quot;, &quot;.prev_items_without_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = (&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2-1).dblclick();
                $('#button_clicked').val('');
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2-1).dblclick();
                $('#button_clicked').val('');
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index-1).dblclick();
        $('#button_clicked').val('');
        return
    })

    $(document).on(&quot;click&quot;, &quot;.next_items_without_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2+1).dblclick();
                $('#button_clicked').val('');
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2+1).dblclick();
                $('#button_clicked').val('');
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index+1).dblclick();
        $('#button_clicked').val('');
        return
    })

    $(document).on(&quot;click&quot;, &quot;.next_items_without_assemblies&quot;, function(e){
        e.preventDefault();
        if($(this).data('check_url') == 'assembly'){
            update_item_details($('#id_update_item_form'));
        }
        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2+1).dblclick();
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2+1).dblclick();
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index+1).dblclick();
        return
    })






/html[1]
  @keyframes intercom-lightweight-app-launcher {
    from {
      opacity: 0;
      transform: scale(0.5);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  @keyframes intercom-lightweight-app-gradient {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes intercom-lightweight-app-messenger {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .intercom-lightweight-app {
    position: fixed;
    z-index: 2147483001;
    width: 0;
    height: 0;
    font-family: intercom-font, &quot;Helvetica Neue&quot;, &quot;Apple Color Emoji&quot;, Helvetica, Arial, sans-serif;
  }

  .intercom-lightweight-app-gradient {
    position: fixed;
    z-index: 2147483002;
    width: 500px;
    height: 500px;
    bottom: 0;
    right: 0;
    pointer-events: none;
    background: radial-gradient(
      ellipse at bottom right,
      rgba(29, 39, 54, 0.16) 0%,
      rgba(29, 39, 54, 0) 72%);
    animation: intercom-lightweight-app-gradient 200ms ease-out;
  }

  .intercom-lightweight-app-launcher {
    position: fixed;
    z-index: 2147483003;
    bottom: 20px;
    right: 100px;
    width: 60px;
    height: 60px;
    border-radius: 50%;
    background: #006eb3;
    cursor: pointer;
    box-shadow: 0 1px 6px 0 rgba(0, 0, 0, 0.06), 0 2px 32px 0 rgba(0, 0, 0, 0.16);
    animation: intercom-lightweight-app-launcher 250ms ease;
  }

  .intercom-lightweight-app-launcher:focus {
    outline: none;
    
  }

  .intercom-lightweight-app-launcher-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
    top: 0;
    left: 0;
    width: 60px;
    height: 60px;
    transition: transform 100ms linear, opacity 80ms linear;
  }

  .intercom-lightweight-app-launcher-icon-open {
    
        opacity: 1;
        transform: rotate(0deg) scale(1);
      
  }

  .intercom-lightweight-app-launcher-icon-open svg {
    width: 28px;
    height: 32px;
  }

  .intercom-lightweight-app-launcher-icon-open svg path {
    fill: rgb(255, 255, 255);
  }

  .intercom-lightweight-app-launcher-icon-self-serve {
    
        opacity: 1;
        transform: rotate(0deg) scale(1);
      
  }

  .intercom-lightweight-app-launcher-icon-self-serve svg {
    height: 56px;
  }

  .intercom-lightweight-app-launcher-icon-self-serve svg path {
    fill: rgb(255, 255, 255);
  }

  .intercom-lightweight-app-launcher-custom-icon-open {
    max-height: 36px;
    max-width: 36px;
    
        opacity: 1;
        transform: rotate(0deg) scale(1);
      
  }

  .intercom-lightweight-app-launcher-icon-minimize {
    
        opacity: 0;
        transform: rotate(-60deg) scale(0);
      
  }

  .intercom-lightweight-app-launcher-icon-minimize svg {
    width: 16px;
  }

  .intercom-lightweight-app-launcher-icon-minimize svg path {
    fill: rgb(255, 255, 255);
  }

  .intercom-lightweight-app-messenger {
    position: fixed;
    z-index: 2147483003;
    overflow: hidden;
    background-color: white;
    animation: intercom-lightweight-app-messenger 250ms ease-out;
    
        width: 376px;
        height: calc(100% - 120px);
        max-height: 704px;
        min-height: 250px;
        right: 100px;
        bottom: 100px;
        box-shadow: 0 5px 40px rgba(0,0,0,0.16);
        border-radius: 8px;
      
  }

  .intercom-lightweight-app-messenger-header {
    height: 75px;
    background: linear-gradient(
      135deg,
      rgb(0, 46, 114) 0%,
      rgb(0, 5, 12) 100%
    );
  }

  @media print {
    .intercom-lightweight-app {
      display: none;
    }
  }
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
